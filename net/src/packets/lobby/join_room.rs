use crate::definition_packet;

definition_packet!(
    #[req]
    pub struct LobbyRoomJoinReq {
        pub key: String,
    }

    #[res]
    pub enum LobbyRoomJoinRes {
        Success,
        RoomNotFound,
    }
);
