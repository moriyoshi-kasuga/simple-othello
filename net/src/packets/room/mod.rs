use crate::{
    definition_packets,
    packets::room::{
        color::{RoomChoiceColorBroadcast, RoomChoiceColorReq, RoomChoiceColorRes},
        join::RoomUserJoinBroadcast,
        leave::RoomUserLeaveBroadcast,
    },
};

pub mod color;
pub mod join;
pub mod leave;

definition_packets!(
    pub enum RoomRequestPacket {
        RoomChoiceColor(RoomChoiceColorReq) = 0,
    }
);

definition_packets!(
    pub enum RoomResponsePacket {
        RoomUserJoinBroadcast(RoomUserJoinBroadcast) = 0,
        RoomChoiceColor(RoomChoiceColorRes) = 1,
        RoomChoiceColorBroadcast(RoomChoiceColorBroadcast) = 2,
        RoomUserLeaveBroadcast(RoomUserLeaveBroadcast) = 3,
    }
);
