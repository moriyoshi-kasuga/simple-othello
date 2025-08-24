use crate::{
    definition_packets,
    models::{
        login::LoginReq,
        room::{RoomCreateReq, RoomJoinReq},
    },
};

definition_packets!(
    pub enum RequestPacket {
        Login(LoginReq) = 0,
        RoomCreate(RoomCreateReq) = 1,
        RoomJoin(RoomJoinReq) = 2,
    }
);
