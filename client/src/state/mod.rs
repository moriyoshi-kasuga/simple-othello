pub mod connection;

#[cfg(feature = "dev")]
const WEBSOCKET_URL: &str = "ws://127.0.0.1:3000/ws";
#[cfg(not(feature = "dev"))]
const WEBSOCKET_URL: &str = concat!("ws://", env!("OTHELLO_HOST"), "/ws");
