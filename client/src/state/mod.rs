use crate::state::connection::Connection;

pub mod connection;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub connection: Connection,
}

pub fn use_app_state() -> AppState {
    dioxus::hooks::use_context()
}
