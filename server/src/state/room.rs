use logic::{OthelloBoard, OthelloColor};
use std::{borrow::Borrow, sync::Arc};

use enum_table::{EnumTable, Enumable};
use net::{
    EncodablePacket,
    packets::room::{join::RoomUserJoinBroadcast, leave::RoomUserLeaveBroadcast},
};
use tokio::sync::RwLock;

use crate::state::user::User;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomKey(Arc<String>);

impl RoomKey {
    pub fn new(key: String) -> Self {
        Self(Arc::new(key))
    }
}

impl AsRef<str> for RoomKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for RoomKey {
    fn eq(&self, other: &str) -> bool {
        *self.0 == other
    }
}

impl PartialEq<RoomKey> for str {
    fn eq(&self, other: &RoomKey) -> bool {
        self == *other.0
    }
}

impl Borrow<str> for RoomKey {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct Room {
    pub key: RoomKey,
    pub users: Arc<RwLock<Vec<User>>>,
    pub state: Arc<RwLock<RoomState>>,
}

impl Room {
    pub fn new(key: RoomKey) -> Self {
        Self {
            key,
            users: Arc::new(RwLock::new(Vec::new())),
            state: Arc::new(RwLock::new(RoomState::Waiting {
                players: EnumTable::default(),
            })),
        }
    }

    pub async fn broadcast<P: EncodablePacket>(&self, packet: &P) {
        let users = self.users.read().await;
        let send_futures: Vec<_> = users
            .iter()
            .map(|user| user.connection.send(packet))
            .collect();
        futures::future::join_all(send_futures).await;
    }

    pub async fn add_user(&self, user: User) {
        let res = RoomUserJoinBroadcast {
            uid: user.uid,
            username: (*user.username).clone(),
        };
        self.broadcast(&res).await;

        user.join_room(self.key.clone()).await;
        let mut users = self.users.write().await;
        users.push(user);
    }

    pub async fn leave_user(&self, uid: uid::Uid) {
        let user = {
            let mut users = self.users.write().await;
            users
                .iter()
                .position(|u| u.uid == uid)
                .map(|pos| users.remove(pos))
        };

        if let Some(user) = user {
            // Broadcast leave message to remaining users in parallel
            let res = RoomUserLeaveBroadcast { uid };
            self.broadcast(&res).await;
            self.unset_player_color(&user).await;
        }
    }

    pub async fn set_player_color(&self, user: &User, color: OthelloColor) -> bool {
        let mut state = self.state.write().await;
        match &mut *state {
            RoomState::Waiting { players } => {
                if players[color].is_none()
                    && !players
                        .iter()
                        .any(|(_, u)| u.as_ref().is_some_and(|u| u.uid == user.uid))
                {
                    players[color] = Some(user.clone());
                    true
                } else {
                    false
                }
            }
            RoomState::InGame { .. } => false,
        }
    }

    pub async fn unset_player_color(&self, user: &User) {
        let mut state = self.state.write().await;
        match &mut *state {
            RoomState::Waiting { players } => {
                for (_, u) in players.iter_mut() {
                    if u.as_ref().is_some_and(|u| u.uid == user.uid) {
                        *u = None;
                    }
                }
            }
            RoomState::InGame { .. } => {}
        }
    }

    pub async fn start_game(&self) -> bool {
        let mut state = self.state.write().await;
        match &*state {
            RoomState::Waiting { players } => {
                let filled_players =
                    EnumTable::checked_new_with_fn(|color| players.get(color).as_ref().cloned());
                match filled_players {
                    Ok(players) => {
                        let game = OthelloBoard::new();
                        *state = RoomState::InGame { players, game };
                        true
                    }
                    Err(_) => false,
                }
            }
            RoomState::InGame { .. } => false,
        }
    }

    pub async fn is_empty(&self) -> bool {
        let users = self.users.read().await;
        users.is_empty()
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
