use core::{OthelloBoard, OthelloColor};
use std::sync::Arc;

use enum_table::{EnumTable, Enumable};
use tokio::sync::RwLock;

use crate::state::user::User;

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
    pub users: Arc<RwLock<Vec<User>>>,
    pub state: Arc<RwLock<RoomState>>,
}

impl Room {
    pub fn new(key: RoomKey) -> Self {
        Self {
            key: Arc::new(key),
            users: Arc::new(RwLock::new(Vec::new())),
            state: Arc::new(RwLock::new(RoomState::Waiting {
                players: EnumTable::default(),
            })),
        }
    }

    pub async fn add_user(&self, user: User) {
        let mut users = self.users.write().await;
        users.push(user);
    }
}

pub enum RoomState {
    Waiting {
        players: EnumTable<OthelloColor, Option<User>, { OthelloColor::COUNT }>,
    },
    InGame {
        players: EnumTable<OthelloColor, User, { OthelloColor::COUNT }>,
        game: OthelloBoard,
    },
}
