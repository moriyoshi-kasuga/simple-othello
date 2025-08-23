use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures_util::SinkExt;
use net::models::login::LoginRequest;

use crate::state::{
    AppState,
    connection::{Connection, ReceiveValue},
};

pub async fn handle_socket(mut socket: WebSocket, state: AppState) {
    log::info!("New WebSocket connection established");

    async fn close_socket(socket: &mut WebSocket) {
        if let Err(e) = socket.close().await {
            log::error!("Failed to close socket: {:?}", e);
        }
    }

    let Some(Ok(Message::Text(msg))) = socket.recv().await else {
        close_socket(&mut socket).await;
        return;
    };

    let login_req: LoginRequest = match serde_json::from_str(&msg) {
        Ok(req) => req,
        Err(e) => {
            log::error!("Failed to parse login request: {:?}", e);
            close_socket(&mut socket).await;
            return;
        }
    };
    log::info!("User '{}' logged in", login_req.username);

    let connection = Connection::new(login_req.username, socket);
    state.add_connection(connection.clone()).await;

    while let Some::<ReceiveValue>(value) = connection.receive().await {
        let ReceiveValue::Text(msg) = value else {
            continue;
        };
        handle_inner(msg, &connection).await;
    }

    state.close_connection(connection.uid).await;
}

async fn handle_inner(msg: Utf8Bytes, connection: &Connection) {
    let _ = msg;
    let _ = connection;
    // ...
}
