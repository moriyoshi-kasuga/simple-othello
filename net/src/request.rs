use crate::{
    definition_packets,
    models::room::{RoomCreateReq, RoomJoinReq},
};

definition_packets!(
    pub enum RequestPacket {
        RoomCreate(RoomCreateReq) = 1,
        RoomJoin(RoomJoinReq) = 2,
    }
);
