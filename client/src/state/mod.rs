use dioxus::prelude::*;
use net::packets::UserData;

use crate::state::connection::Connection;

pub mod connection;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub connection: Connection,
    pub user_data: UserData,
}

pub fn use_app_state() -> AppState {
    dioxus::hooks::use_context()
}

pub fn use_connection() -> Connection {
    dioxus::hooks::use_context()
}

#[derive(PartialEq, Props, Clone)]
pub struct AppStateProviderProps {
    pub user_data: UserData,
    pub children: Element,
}

#[component]
pub fn AppStateProvider(props: AppStateProviderProps) -> Element {
    let connection = use_connection();
    let app_state = AppState {
        connection,
        user_data: props.user_data,
    };
    use_context_provider(|| app_state);
    props.children
}
