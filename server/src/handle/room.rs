use net::packets::room::RoomRequestPacket;

use crate::state::{AppState, user::User};

pub async fn handle_room(state: &AppState, user: &User, req: RoomRequestPacket) {}
