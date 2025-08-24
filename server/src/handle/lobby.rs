use net::packets::lobby::LobbyRequestPacket;

use crate::state::{AppState, user::User};

pub async fn handle_lobby(state: &AppState, user: &User, req: LobbyRequestPacket) {
    match req {
        LobbyRequestPacket::RoomCreate(lobby_room_create_req) => todo!(),
        LobbyRequestPacket::RoomJoin(lobby_room_join_req) => todo!(),
    }
}
