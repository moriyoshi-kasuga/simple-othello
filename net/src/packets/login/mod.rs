use uid::Uid;

use crate::{definition_packet, definition_packets};

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

definition_packets!(
    pub enum LoginRequestPacket {
        Login(LoginReq) = 0,
    }
);

definition_packets!(
    pub enum LoginResponsePacket {
        Login(LoginRes) = 0,
    }
);
