use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uid::Uid;

use crate::state::{
    room::{Room, RoomKey},
    user::User,
};

pub mod connection;
pub mod room;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<HashMap<Uid, User>>>,
    rooms: Arc<RwLock<HashMap<RoomKey, Room>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            users: Default::default(),
            rooms: Default::default(),
        }
    }

    pub async fn add_user(&self, user: User) {
        let mut users = self.users.write().await;
        users.insert(user.uid, user);
    }

    pub async fn get_user(&self, uid: Uid) -> Option<User> {
        let users = self.users.read().await;
        users.get(&uid).cloned()
    }

    pub async fn close_user(&self, uid: Uid) {
        let mut users = self.users.write().await;
        if let Some(user) = users.remove(&uid) {
            user.connection.close().await;
        }
    }

    pub async fn add_room(&self, room: Room) {
        let mut rooms = self.rooms.write().await;
        rooms.insert(room.key.clone(), room);
    }

    pub async fn get_room(&self, key: &str) -> Option<Room> {
        let rooms = self.rooms.read().await;
        rooms.get(key).cloned()
    }
}
