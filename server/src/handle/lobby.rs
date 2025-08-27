use net::packets::lobby::{
    LobbyRequestPacket, create_room::LobbyRoomCreateRes, join_room::LobbyRoomJoinRes,
};

use crate::state::{
    AppState,
    room::{Room, RoomKey},
    user::User,
};

pub async fn handle_lobby(state: &AppState, user: &User, req: LobbyRequestPacket) {
    match req {
        LobbyRequestPacket::RoomCreate(req) => {
            // Validate room key
            if req.key.is_empty() {
                log::warn!("User '{}' attempted to create room with empty key", user.username);
                return;
            }
            if req.key.len() > 32 {
                log::warn!("User '{}' attempted to create room with key longer than 32 chars", user.username);
                return;
            }
            
            // Check if room already exists
            if state.get_room(&req.key).await.is_some() {
                log::warn!("User '{}' attempted to create room with existing key '{}'", user.username, req.key);
                return;
            }
            
            let key = RoomKey::new(req.key.clone());
            let room = Room::new(key);
            room.add_user(user.clone()).await;
            state.add_room(room).await;
            
            log::info!("User '{}' created room '{}'", user.username, req.key);
            
            let res = LobbyRoomCreateRes {};
            user.connection.send(&res).await;
        }
        LobbyRequestPacket::RoomJoin(req) => {
            // Validate room key
            if req.key.is_empty() {
                log::warn!("User '{}' attempted to join room with empty key", user.username);
                user.connection.send(&LobbyRoomJoinRes::RoomNotFound).await;
                return;
            }
            
            let Some::<Room>(room) = state.get_room(&req.key).await else {
                log::info!("User '{}' attempted to join non-existent room '{}'", user.username, req.key);
                user.connection.send(&LobbyRoomJoinRes::RoomNotFound).await;
                return;
            };
            
            let res = LobbyRoomJoinRes::Success {
                users: room
                    .users
                    .read()
                    .await
                    .iter()
                    .map(|u| u.to_data())
                    .collect(),
            };

            room.add_user(user.clone()).await;
            user.connection.send(&res).await;
            
            log::info!("User '{}' joined room '{}'", user.username, req.key);
        }
    }
}
