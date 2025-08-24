use std::rc::Rc;

use futures::{
    SinkExt, StreamExt,
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use gloo_net::websocket::{Message, WebSocketError, futures::WebSocket};
use net::ReqPacket;

use crate::state::WEBSOCKET_URL;

#[derive(Clone)]
pub struct Connection {
    tx: Rc<Mutex<SplitSink<WebSocket, Message>>>,
    rx: Rc<Mutex<SplitStream<WebSocket>>>,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.tx, &other.tx) && Rc::ptr_eq(&self.rx, &other.rx)
    }
}

pub enum ReceiveValue {
    Binary(Vec<u8>),
    Other,
}

impl Connection {
    pub fn new() -> Option<Self> {
        let websocket = match WebSocket::open(WEBSOCKET_URL) {
            Ok(ws) => ws,
            Err(e) => {
                log::error!("Failed to connect to WebSocket: {:?}", e);
                return None;
            }
        };

        Some(Self::new_websocket(websocket))
    }

    fn new_websocket(websocket: WebSocket) -> Self {
        let (tx, rx) = websocket.split();
        Self {
            tx: Rc::new(Mutex::new(tx)),
            rx: Rc::new(Mutex::new(rx)),
        }
    }

    pub async fn send<Req: ReqPacket>(&self, req: Req) -> Result<(), WebSocketError> {
        if let Some(buf) = req.encode() {
            let msg = Message::Bytes(buf);
            self.tx.lock().await.send(msg).await
        } else {
            let error = js_sys::Error::new("Failed to encode request");
            Err(WebSocketError::MessageSendError(error.into()))
        }
    }

    pub async fn receive(&self) -> Option<ReceiveValue> {
        let receive = self.rx.lock().await.next().await?;
        match receive {
            Ok(Message::Bytes(data)) => Some(ReceiveValue::Binary(data)),
            Ok(_) => Some(ReceiveValue::Other),
            Err(e) => {
                log::error!("WebSocket receive error: {:?}", e);
                None
            }
        }
    }
}
