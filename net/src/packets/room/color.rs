use logic::OthelloColor;

use uid::Uid;

use crate::definition_packet;

definition_packet!(
    #[req]
    pub struct RoomChoiceColorReq {
        pub color: OthelloColor,
    }

    #[res]
    pub struct RoomChoiceColorRes {
        pub success: bool,
    }

    #[res]
    pub struct RoomChoiceColorBroadcast {
        pub uid: Uid,
        pub color: OthelloColor,
    }
);
