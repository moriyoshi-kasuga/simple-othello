use net::packets::room::{
    RoomRequestPacket,
    color::{RoomChoiceColorBroadcast, RoomChoiceColorRes},
};

use crate::state::{AppState, room::Room, user::User};

pub async fn handle_room(_state: &AppState, user: &User, req: RoomRequestPacket) {
    let Some::<Room>(room) = user.get_room().await else {
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
            for u in room.users.read().await.iter() {
                if u.uid != user.uid {
                    u.connection
                        .send(&RoomChoiceColorBroadcast {
                            uid: user.uid,
                            color: req.color,
                        })
                        .await;
                }
            }
        }
    }
}
