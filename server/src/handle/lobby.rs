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
            let key = RoomKey::new(req.key);
            let room = Room::new(key);
            room.add_user(user.clone()).await;
            state.add_room(room).await;
            let res = LobbyRoomCreateRes {};
            user.connection.send(&res).await;
        }
        LobbyRequestPacket::RoomJoin(req) => {
            let Some::<Room>(room) = state.get_room(&req.key).await else {
                user.connection.send(&LobbyRoomJoinRes::RoomNotFound).await;
                return;
            };
            room.add_user(user.clone()).await;
            let res = LobbyRoomJoinRes::Success {
                users: room
                    .users
                    .read()
                    .await
                    .iter()
                    .map(|u| u.to_data())
                    .collect(),
            };
            user.connection.send(&res).await;
        }
    }
}
