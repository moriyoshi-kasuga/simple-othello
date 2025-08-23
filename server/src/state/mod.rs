use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uid::Uid;

use crate::state::connection::Connection;

pub mod connection;

#[derive(Clone)]
pub struct AppState {
    connections: Arc<RwLock<HashMap<Uid, Connection>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            connections: Default::default(),
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
}
