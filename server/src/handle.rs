use axum::extract::ws::WebSocket;
use net::{
    models::{
        login::LoginReq,
        room::{RoomJoinRes, RoomOtherJoinedRes},
    },
    request::RequestPacket,
};
use uid::Uid;

use crate::state::{
    AppState,
    connection::{Connection, ReceiveValue},
    room::{Room, RoomKey},
    user::User,
};

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let user = {
        let connection = Connection::new(socket);

        let Some::<LoginReq>(login_req) = connection.receive_special::<LoginReq>().await else {
            log::warn!("Failed to receive login request");
            connection.close().await;
            return;
        };

        User::new(Uid::new(), login_req.username, connection)
    };

    state.add_user(user.clone()).await;

    log::info!("User '{}' logged in", user.username);

    while let Some::<ReceiveValue>(value) = user.connection.receive().await {
        let ReceiveValue::Binary(msg) = value else {
            continue;
        };
        let Some(req) = RequestPacket::decode(&msg) else {
            user.connection.close().await;
            return;
        };
        handle_inner(&state, &user, req).await;
    }

    log::info!("User '{}' disconnected", user.username);
}

async fn handle_inner(state: &AppState, user: &User, req: RequestPacket) {
    match req {
        RequestPacket::RoomCreate(room_create_req) => {
            log::info!(
                "User '{}' requested to create room with name '{}'",
                user.username,
                room_create_req.key,
            );
            let key = RoomKey::new(room_create_req.key);
            let room = Room::new(key);
            room.add_user(user.clone()).await;
            state.add_room(room).await;
        }
        RequestPacket::RoomJoin(room_join_req) => {
            log::info!(
                "User '{}' requested to join room with name '{}'",
                user.username,
                room_join_req.key,
            );
            let key = RoomKey::new(room_join_req.key);
            let Some::<Room>(room) = state.get_room(&key).await else {
                user.connection.send(&RoomJoinRes::RoomNotFound).await;
                return;
            };
            {
                let res = RoomOtherJoinedRes {
                    username: (*user.username).clone(),
                };
                let connections = room.users.read().await;
                for conn in &*connections {
                    conn.connection.send(&res).await;
                }
            }
            room.add_user(user.clone()).await;
            user.connection.send(&RoomJoinRes::Success).await;
        }
        RequestPacket::Login(_) => {
            log::warn!(
                "User '{}' sent login request after logged in",
                user.username
            );
            user.connection.close().await;
        }
    }
}
