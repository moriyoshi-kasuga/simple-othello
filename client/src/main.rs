use dioxus::prelude::*;
use net::packets::UserData;

use crate::{
    pages::login::Login,
    state::{AppState, connection::Connection},
};

mod components;
mod pages;
pub mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Connection::new());
    let mut uesr_data = use_signal(|| None::<UserData>);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            match &*uesr_data.read() {
                Some(user_data) => rsx! {
                    div {
                        // TODO: Add more components here
                        div { class: "p-4 text-center text-2xl font-bold", "Welcome, {user_data.username}!" }
                    }
                },
                None => rsx! {
                    Login {
                        on_login: move |data| {
                            tracing::info!("User logged in: {:?}", data);
                            uesr_data.set(Some(data));
                        },
                    }
                },
            }
        }
    }
}
