use crate::{definition_packet, definition_packets, packets::room::join::RoomUserJoinRes};

pub mod join;

definition_packet!(
    #[req]
    pub struct TempRequest {
        pub temp: String,
    }
);

definition_packets!(
    pub enum RoomRequestPacket {
        /// Temporary packet for testing room packet structure
        Temp(TempRequest) = 0,
    }
);

definition_packets!(
    pub enum RoomResponsePacket {
        RoomUserJoin(RoomUserJoinRes) = 0,
    }
);
