use dioxus::prelude::*;

use crate::state::use_app_state;

#[component]
pub fn Lobby() -> Element {
    let state = use_app_state();
    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-100",
            p { class: "text-gray-700 text-xl", "Welcome, {state.user_data.username}!" }
        }
    }
}
