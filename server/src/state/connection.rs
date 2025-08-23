use std::sync::{Arc, atomic::AtomicBool};

use axum::{body::Bytes, extract::ws::WebSocket};
use futures_util::SinkExt;
use net::ResPacket;
use tokio::sync::Mutex;
use uid::Uid;

#[derive(Clone)]
pub struct Connection {
    pub uid: Uid,
    pub username: Arc<String>,
    pub websocket: Arc<Mutex<WebSocket>>,

    interrupt_notify: Arc<tokio::sync::Notify>,
    closed: Arc<AtomicBool>,
}

pub enum ReceiveValue {
    Binary(Bytes),
    Interrupt,
    Other,
}

impl Connection {
    pub fn new(username: String, websocket: WebSocket) -> Self {
        Self {
            uid: Uid::new(),
            username: Arc::new(username),
            websocket: Arc::new(Mutex::new(websocket)),

            interrupt_notify: Default::default(),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// If `None` is returned, the connection should be closed.
    pub async fn receive(&self) -> Option<ReceiveValue> {
        let mut wc = tokio::select! {
            _ = self.interrupt_notify.notified() => {
                return Some(ReceiveValue::Interrupt);
            }
            wc = self.websocket.lock() => {
                wc
            }
        };
        let msg = tokio::select! {
            _ = self.interrupt_notify.notified() => {
                return Some(ReceiveValue::Interrupt);
            }
            msg = wc.recv() => {
                drop(wc);
                msg
            }
        };

        match msg {
            Some(Ok(axum::extract::ws::Message::Binary(t))) => Some(ReceiveValue::Binary(t)),
            Some(Ok(axum::extract::ws::Message::Close(_))) => {
                log::info!("WebSocket closed by client for user '{}'", self.username);
                None
            }
            Some(Ok(_)) => Some(ReceiveValue::Other),
            Some(Err(e)) => {
                log::error!("WebSocket error for user '{}': {:?}", self.username, e);
                None
            }
            None => {
                log::info!("WebSocket closed for user '{}'", self.username);
                None
            }
        }
    }

    pub async fn raw_send(&self, res: Vec<u8>) -> Result<(), axum::Error> {
        self.interrupt_notify.notify_one();
        let mut ws = self.websocket.lock().await;
        if let Err(e) = ws.send(axum::extract::ws::Message::binary(res)).await {
            log::error!("Failed to send message to {}: {:?}", self.username, e);
            return Err(e);
        }
        Ok(())
    }

    pub async fn raw_send_or_close(&self, res: Vec<u8>) {
        if self.raw_send(res).await.is_err() {
            self.close().await;
        }
    }

    pub async fn send<Res: ResPacket>(&self, res: &Res) {
        if let Some(buf) = res.encode() {
            self.raw_send_or_close(buf).await;
        } else {
            log::error!(
                "Failed to encode response packet for user '{}'",
                self.username
            );
            self.close().await;
        }
    }

    pub async fn close(&self) {
        if self.closed.swap(true, std::sync::atomic::Ordering::SeqCst) {
            // Already closed
            return;
        }
        self.interrupt_notify.notify_one();
        let mut ws = self.websocket.lock().await;
        if let Err(e) = ws.close().await {
            log::error!(
                "Failed to close WebSocket for user '{}': {:?}",
                self.username,
                e
            );
            return;
        }

        log::info!("WebSocket closed for user '{}'", self.username);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::Relaxed)
    }
}
