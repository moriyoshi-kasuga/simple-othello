use crate::{definition_packet, packets::UserData};

definition_packet!(
    #[req]
    pub struct LobbyRoomJoinReq {
        pub key: String,
    }

    #[res]
    pub enum LobbyRoomJoinRes {
        Success { users: Vec<UserData> },
        RoomNotFound,
    }
);
