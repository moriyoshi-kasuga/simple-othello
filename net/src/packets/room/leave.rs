use uid::Uid;

use crate::definition_packet;

definition_packet!(
    #[res]
    pub struct RoomUserLeaveBroadcast {
        pub uid: Uid,
    }
);
