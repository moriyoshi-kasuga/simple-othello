use dioxus::prelude::*;

pub mod lobby;
pub mod login;

#[component]
pub fn PageRouter() -> Element {
    rsx! {
        lobby::Lobby {}
    }
}
