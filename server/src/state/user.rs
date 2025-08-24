use std::sync::Arc;

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
        Self {
            uid,
            username: Arc::new(username),
            connection,
        }
    }
}
