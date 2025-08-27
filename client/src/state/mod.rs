use net::packets::UserData;

use crate::state::connection::Connection;

pub mod connection;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub connection: Connection,
    pub use_data: UserData,
}

pub fn use_app_state() -> AppState {
    dioxus::hooks::use_context()
}

pub fn use_connection() -> Connection {
    dioxus::hooks::use_context()
}
