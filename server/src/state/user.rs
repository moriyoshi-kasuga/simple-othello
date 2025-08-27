use std::sync::Arc;

use net::state::ConnState;
use tokio::sync::RwLock;
use uid::Uid;

use crate::state::{connection::Connection, room::RoomKey};

#[derive(Clone)]
pub struct User {
    pub uid: Uid,
    pub username: Arc<String>,
    pub connection: Connection,
    room_key: Arc<RwLock<Option<RoomKey>>>,
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

    pub async fn join_room(&self, room_key: RoomKey) {
        let mut rk = self.room_key.write().await;
        *rk = Some(room_key);
        self.connection.set_conn_state(ConnState::Room);
    }

    pub async fn leave_room(&self) {
        let mut rk = self.room_key.write().await;
        *rk = None;
        self.connection.set_conn_state(ConnState::Lobby);
    }

    pub async fn get_room_key(&self) -> Option<RoomKey> {
        let room_key = self.room_key.read().await;
        room_key.clone()
    }

    pub fn to_data(&self) -> net::packets::UserData {
        net::packets::UserData {
            uid: self.uid,
            username: (*self.username).clone(),
        }
    }
}
