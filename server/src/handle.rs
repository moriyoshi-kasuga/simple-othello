use std::ops::Deref;

use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket},
};
use futures_util::SinkExt;
use net::{
    login::LoginRequest,
    models::room::{RoomJoinRes, RoomOtherJoinedRes},
    request::RequestPacket,
};

use crate::state::{
    AppState,
    connection::{Connection, ReceiveValue},
    room::{Room, RoomKey},
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
        let ReceiveValue::Binary(msg) = value else {
            continue;
        };
        handle_inner(&state, &connection, msg).await;
    }

    state.close_connection(connection.uid).await;
}

async fn handle_inner(state: &AppState, connection: &Connection, msg: Bytes) {
    let Some(req) = RequestPacket::decode(&msg) else {
        log::warn!(
            "Failed to decode request packet from user '{}'",
            connection.username
        );
        connection.close().await;
        return;
    };

    match req {
        RequestPacket::RoomCreate(room_create_req) => {
            log::info!(
                "User '{}' requested to create room with name '{}'",
                connection.username,
                room_create_req.key,
            );
            let key = RoomKey::new(room_create_req.key);
            let room = Room::new(key);
            room.add_connection(connection.clone()).await;
            state.add_room(room).await;
        }
        RequestPacket::RoomJoin(room_join_req) => {
            log::info!(
                "User '{}' requested to join room with name '{}'",
                connection.username,
                room_join_req.key,
            );
            let key = RoomKey::new(room_join_req.key);
            let Some::<Room>(room) = state.get_room(&key).await else {
                connection.send(RoomJoinRes::RoomNotFound).await;
                return;
            };
            {
                let res = RoomOtherJoinedRes {
                    username: connection.username.deref().to_string(),
                };
                let connections = room.connections.read().await;
                for conn in &*connections {
                    conn.send(res.clone()).await;
                }
            }
            room.add_connection(connection.clone()).await;
            connection.send(RoomJoinRes::Success).await;
        }
    }
}
