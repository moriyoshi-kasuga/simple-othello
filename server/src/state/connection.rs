use std::sync::{Arc, atomic::AtomicBool};

use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket},
};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use net::{ReqPacket, ResPacket};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Connection {
    tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    rx: Arc<Mutex<SplitStream<WebSocket>>>,

    closed: Arc<AtomicBool>,
}

pub enum ReceiveValue {
    Binary(Bytes),
    Interrupt,
    Other,
}

impl Connection {
    pub fn new(websocket: WebSocket) -> Self {
        let (tx, rx) = websocket.split();

        Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),

            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// If `None` is returned, the connection should be closed.
    pub async fn receive(&self) -> Option<ReceiveValue> {
        let msg = {
            let mut rx = self.rx.lock().await;
            rx.next().await
        };

        match msg {
            Some(Ok(Message::Binary(t))) => Some(ReceiveValue::Binary(t)),
            Some(Ok(Message::Close(_))) => None,
            Some(Ok(_)) => Some(ReceiveValue::Other),
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub async fn receive_special<Req: ReqPacket>(&self) -> Option<Req> {
        let req = self.receive().await?;
        let ReceiveValue::Binary(buf) = req else {
            return None;
        };
        Req::decode(&buf)
    }

    pub async fn raw_send(&self, res: Vec<u8>) -> Result<(), axum::Error> {
        let mut tx = self.tx.lock().await;
        tx.send(Message::binary(res)).await
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
            self.close().await;
        }
    }

    pub async fn close(&self) {
        if self.closed.swap(true, std::sync::atomic::Ordering::Relaxed) {
            // Already closed
            return;
        }
        let mut tx = self.tx.lock().await;
        let _ = tx.close().await;
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::Relaxed)
    }
}
