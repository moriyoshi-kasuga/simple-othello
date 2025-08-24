use std::sync::Arc;

use net::state::ConnState;
use uid::Uid;

use crate::state::connection::Connection;

#[derive(Clone)]
pub struct User {
    pub uid: Uid,
    pub username: Arc<String>,
    pub connection: Connection,
}

impl User {
    pub fn new(uid: Uid, username: String, connection: Connection) -> Self {
        connection.set_conn_state(ConnState::Lobby);
        Self {
            uid,
            username: Arc::new(username),
            connection,
        }
    }
}
