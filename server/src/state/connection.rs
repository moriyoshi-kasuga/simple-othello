use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU8},
};

use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket},
};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use net::{DecodablePacket, EncodablePacket, state::ConnState};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Connection {
    tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    rx: Arc<Mutex<SplitStream<WebSocket>>>,

    conn_state: Arc<AtomicU8>,
    closed: Arc<AtomicBool>,
}

impl Connection {
    pub fn new(websocket: WebSocket) -> Self {
        let (tx, rx) = websocket.split();

        Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),

            conn_state: Arc::new(AtomicU8::new(ConnState::Login as u8)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// If `None` is returned, the connection should be closed.
    pub async fn receive_raw(&self) -> Option<Bytes> {
        let msg = {
            let mut rx = self.rx.lock().await;
            rx.next().await
        };

        match msg {
            Some(Ok(Message::Binary(t))) => Some(t),
            _ => None,
        }
    }

    pub async fn receive<P: DecodablePacket>(&self) -> Option<P> {
        let buf = self.receive_raw().await?;
        P::decode(&buf)
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

    pub async fn send<P: EncodablePacket>(&self, packet: &P) {
        if let Some(buf) = packet.encode() {
            self.raw_send_or_close(buf).await;
        } else {
            self.close().await;
        }
    }

    pub fn get_conn_state(&self) -> ConnState {
        let raw_state = self.conn_state.load(std::sync::atomic::Ordering::Acquire);

        unsafe { std::mem::transmute(raw_state) }
    }

    pub fn set_conn_state(&self, state: ConnState) {
        self.conn_state
            .store(state as u8, std::sync::atomic::Ordering::Release);
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
