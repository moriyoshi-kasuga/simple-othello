use std::{
    rc::Rc,
    sync::{Arc, atomic::AtomicBool},
};

use futures_util::{
    SinkExt, StreamExt,
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use gloo_net::websocket::{Message, futures::WebSocket};
use net::{
    DecodablePacket, EncodablePacket,
    state::{AtomicConnState, ConnState},
};

#[derive(Clone)]
pub struct Connection {
    tx: Rc<Mutex<SplitSink<WebSocket, Message>>>,
    rx: Rc<Mutex<SplitStream<WebSocket>>>,

    conn_state: Arc<AtomicConnState>,
    closed: Arc<AtomicBool>,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.tx, &other.tx) && Rc::ptr_eq(&self.rx, &other.rx)
    }
}

impl Connection {
    #[allow(clippy::new_without_default, clippy::unwrap_used)]
    pub fn new() -> Self {
        let ws_url = if cfg!(feature = "prod") {
            let location = web_sys::window().unwrap().location();
            let host = location.host().unwrap();
            let protocol = if location.protocol().unwrap() == "https:" {
                "wss:"
            } else {
                "ws:"
            };

            format!("{}//{}/ws", protocol, host)
        } else {
            format!(
                "ws://127.0.0.1:{}/ws",
                option_env!("TCP_LISTENER").unwrap_or("3000")
            )
        };
        let ws = WebSocket::open(&ws_url).unwrap();

        Self::new_websocket(ws)
    }

    fn new_websocket(websocket: WebSocket) -> Self {
        let (tx, rx) = websocket.split();
        Self {
            tx: Rc::new(Mutex::new(tx)),
            rx: Rc::new(Mutex::new(rx)),

            conn_state: Arc::new(AtomicConnState::new(ConnState::Login)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn receive_raw(&self) -> Option<Vec<u8>> {
        let msg = {
            let mut rx = self.rx.lock().await;
            rx.next().await
        };

        match msg {
            Some(Ok(Message::Bytes(t))) => Some(t),
            _ => None,
        }
    }

    pub async fn receive<P: DecodablePacket>(&self) -> Option<P> {
        let buf = self.receive_raw().await?;
        P::decode(&buf)
    }

    pub async fn raw_send(&self, res: Vec<u8>) -> Result<(), gloo_net::websocket::WebSocketError> {
        let mut tx = self.tx.lock().await;
        tx.send(Message::Bytes(res)).await
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
        self.conn_state.load()
    }

    pub fn set_conn_state(&self, state: ConnState) {
        self.conn_state.store(state);
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
