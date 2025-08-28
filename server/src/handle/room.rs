use net::packets::room::{
    RoomRequestPacket,
    color::{RoomChoiceColorBroadcast, RoomChoiceColorRes},
};

use crate::state::{AppState, user::User};

pub async fn handle_room(state: &AppState, user: &User, req: RoomRequestPacket) {
    let Some(room_key) = user.get_room_key().await else {
        log::error!(
            "User {} tried to perform room action without being in a room",
            user.uid
        );
        user.connection.close().await;
        return;
    };

    let Some(room) = state.get_room(room_key.as_ref()).await else {
        log::error!("Room {} not found for user {}", room_key.as_ref(), user.uid);
        user.connection.close().await;
        return;
    };
    match req {
        RoomRequestPacket::RoomChoiceColor(req) => {
            let is_success = room.set_player_color(user, req.color).await;
            user.connection
                .send(&RoomChoiceColorRes {
                    success: is_success,
                })
                .await;

            // Broadcast color choice to other users in parallel
            let broadcast = RoomChoiceColorBroadcast {
                uid: user.uid,
                color: req.color,
            };

            room.broadcast(&broadcast).await;
        }
    }
}
