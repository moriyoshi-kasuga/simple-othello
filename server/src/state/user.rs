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
    room: Arc<RwLock<Option<Room>>>,
}

impl User {
    pub fn new(uid: Uid, username: String, connection: Connection) -> Self {
        connection.set_conn_state(ConnState::Lobby);
        Self {
            uid,
            username: Arc::new(username),
            connection,
            room: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn join_room(&self, room: Room) {
        let mut rk = self.room.write().await;
        *rk = Some(room);
        self.connection.set_conn_state(ConnState::Room);
    }

    pub async fn leave_room(&self) {
        let mut rk = self.room.write().await;
        if let Some(room) = rk.as_ref() {
            room.leave_user(self.uid).await;
        }
        *rk = None;
        self.connection.set_conn_state(ConnState::Lobby);
    }

    pub async fn get_room(&self) -> Option<Room> {
        let room = self.room.read().await;
        room.clone()
    }

    pub fn to_data(&self) -> net::packets::UserData {
        net::packets::UserData {
            uid: self.uid,
            username: (*self.username).clone(),
        }
    }
}
