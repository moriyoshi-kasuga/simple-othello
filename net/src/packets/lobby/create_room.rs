use crate::definition_packet;

definition_packet!(
    #[req]
    pub struct LobbyRoomCreateReq {
        pub key: String,
    }

    #[res]
    pub struct LobbyRoomCreateRes {}
);


