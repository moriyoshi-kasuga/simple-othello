use crate::{
    definition_packets,
    packets::lobby::{
        create_room::{LobbyRoomCreateReq, LobbyRoomCreateRes},
        join_room::{LobbyRoomJoinReq, LobbyRoomJoinRes},
    },
};

pub mod create_room;
pub mod join_room;

definition_packets!(
    pub enum LobbyRequestPacket {
        RoomCreate(LobbyRoomCreateReq) = 0,
        RoomJoin(LobbyRoomJoinReq) = 1,
    }
);

definition_packets!(
    pub enum LobbyResponsePacket {
        RoomCreate(LobbyRoomCreateRes) = 0,
        RoomJoin(LobbyRoomJoinRes) = 1,
    }
);
