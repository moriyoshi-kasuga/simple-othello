use crate::definition_packet;

definition_packet!(
    #[req]
    pub struct RoomCreateReq {
        pub key: String,
    }

    #[res]
    pub struct RoomCreateRes {}
);

definition_packet!(
    #[req]
    pub struct RoomJoinReq {
        pub key: String,
    }

    #[res]
    pub enum RoomJoinRes {
        Success,
        RoomNotFound,
    }

    #[res]
    pub struct RoomOtherJoinedRes {
        pub username: String,
    }
);
