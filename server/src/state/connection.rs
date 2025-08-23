use std::sync::{Arc, atomic::AtomicBool};

use axum::extract::ws::{Utf8Bytes, WebSocket};
use futures_util::SinkExt;
use net::models::ResPacket;
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
    Text(Utf8Bytes),
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
            Some(Ok(axum::extract::ws::Message::Text(t))) => Some(ReceiveValue::Text(t)),
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

    pub async fn send<Res: ResPacket>(
        &self,
        res: Res,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.interrupt_notify.notify_one();
        let mut ws = self.websocket.lock().await;
        let msg = match serde_json::to_string(&res) {
            Ok(m) => m,
            Err(e) => {
                log::error!(
                    "Failed to serialize response for user '{}': {:?}",
                    self.username,
                    e
                );
                return Err(Box::new(e));
            }
        };
        if let Err(e) = ws.send(axum::extract::ws::Message::text(msg)).await {
            log::error!("Failed to send message to {}: {:?}", self.username, e);
            return Err(Box::new(e));
        }
        Ok(())
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
