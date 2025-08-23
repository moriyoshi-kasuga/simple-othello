use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uid::Uid;

use crate::state::{
    connection::Connection,
    room::{Room, RoomKey},
};

pub mod connection;
pub mod room;

#[derive(Clone)]
pub struct AppState {
    connections: Arc<RwLock<HashMap<Uid, Connection>>>,
    rooms: Arc<RwLock<HashMap<RoomKey, Room>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            connections: Default::default(),
            rooms: Default::default(),
        }
    }

    pub async fn add_connection(&self, connection: Connection) {
        let mut connections = self.connections.write().await;
        connections.insert(connection.uid, connection);
    }

    pub async fn close_connection(&self, uid: Uid) {
        let mut connections = self.connections.write().await;
        if let Some(conn) = connections.remove(&uid) {
            conn.close().await;
        }
    }

    pub async fn get_connection(&self, uid: Uid) -> Option<Connection> {
        let connections = self.connections.read().await;
        connections.get(&uid).cloned()
    }

    pub async fn add_room(&self, room: Room) {
        let mut rooms = self.rooms.write().await;
        rooms.insert((*room.key).clone(), room);
    }
}
