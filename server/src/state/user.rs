use std::sync::Arc;

use net::state::ConnState;
use tokio::sync::RwLock;
use uid::Uid;

use crate::state::{connection::Connection, room::Room};

#[derive(Clone)]
pub struct User {
    pub uid: Uid,
    pub username: Arc<String>,
    pub connection: Connection,
    room_key: Arc<RwLock<Option<Room>>>,
}

impl User {
    pub fn new(uid: Uid, username: String, connection: Connection) -> Self {
        connection.set_conn_state(ConnState::Lobby);
        Self {
            uid,
            username: Arc::new(username),
            connection,
            room_key: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn join_room(&self, room: Room) {
        let mut rk = self.room_key.write().await;
        *rk = Some(room);
        self.connection.set_conn_state(ConnState::Room);
    }

    pub async fn leave_room(&self) {
        let mut rk = self.room_key.write().await;
        if let Some(room) = rk.as_ref() {
            let mut users = room.users.write().await;
            users.retain(|u| u.uid != self.uid);
        }
        *rk = None;
        self.connection.set_conn_state(ConnState::Lobby);
    }

    pub async fn get_room(&self) -> Option<Room> {
        let room = self.room_key.read().await;
        room.clone()
    }

    pub fn to_data(&self) -> net::packets::UserData {
        net::packets::UserData {
            uid: self.uid,
            username: (*self.username).clone(),
        }
    }
}
