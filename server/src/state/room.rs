use core::{OthelloBoard, OthelloColor};
use std::sync::Arc;

use enum_table::{EnumTable, Enumable};
use tokio::sync::RwLock;

use crate::state::connection::Connection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomKey(String);

impl RoomKey {
    pub fn new(key: String) -> Self {
        Self(key)
    }
}

#[derive(Clone)]
pub struct Room {
    pub key: Arc<RoomKey>,
    pub connections: Arc<RwLock<Vec<Connection>>>,
    pub state: Arc<RwLock<RoomState>>,
}

impl Room {
    pub fn new(key: RoomKey) -> Self {
        Self {
            key: Arc::new(key),
            connections: Default::default(),
            state: Arc::new(RwLock::new(RoomState::Waiting {
                players: EnumTable::default(),
            })),
        }
    }

    pub async fn add_connection(&self, connection: Connection) {
        let mut connections = self.connections.write().await;
        connections.push(connection);
    }
}

pub enum RoomState {
    Waiting {
        players: EnumTable<OthelloColor, Option<Connection>, { OthelloColor::COUNT }>,
    },
    InGame {
        players: EnumTable<OthelloColor, Connection, { OthelloColor::COUNT }>,
        game: OthelloBoard,
    },
}
