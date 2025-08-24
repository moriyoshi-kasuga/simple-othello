use uid::Uid;

use crate::definition_packet;

definition_packet!(
    #[req]
    pub struct LoginReq {
        pub username: String,
    }

    #[res]
    pub struct LoginRes {
        pub uid: Uid,
        pub token: String,
    }
);
