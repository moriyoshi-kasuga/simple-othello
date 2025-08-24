use crate::{
    definition_packets,
    models::{
        login::LoginRes,
        room::{RoomCreateRes, RoomJoinRes, RoomOtherJoinedRes},
    },
};

definition_packets!(
    pub enum ResponsePacket {
        Login(LoginRes) = 0,
        RoomCreate(RoomCreateRes) = 1,
        RoomJoin(RoomJoinRes) = 2,
        RoomOtherJoined(RoomOtherJoinedRes) = 3,
    }
);
