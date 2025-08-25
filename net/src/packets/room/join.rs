use uid::Uid;

use crate::definition_packet;

definition_packet!(
    #[res]
    pub struct RoomUserJoinBroadcast {
        pub uid: Uid,
        pub username: String,
    }
);
