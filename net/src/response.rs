use crate::{
    definition_packets,
    models::room::{RoomCreateRes, RoomJoinRes, RoomOtherJoinedRes},
};

definition_packets!(
    pub enum ResponsePacket {
        RoomCreate(RoomCreateRes) = 1,
        RoomJoin(RoomJoinRes) = 2,
        RoomOtherJoined(RoomOtherJoinedRes) = 3,
    }
);
