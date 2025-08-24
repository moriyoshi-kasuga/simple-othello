use uid::Uid;

use crate::definition_packet;

definition_packet!(
    #[res]
    pub struct RoomUserJoinRes {
        pub uid: Uid,
        pub username: String,
    }
);
