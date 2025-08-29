use dioxus::prelude::*;
use net::packets::UserData;

use crate::{
    pages::{PageRouter, login::Login},
    state::{AppStateProvider, connection::Connection},
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
    use_context_provider(Connection::new);
    let mut uesr_data = use_signal(|| None::<UserData>);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            match &*uesr_data.read() {
                Some(user_data) => rsx! {
                    AppStateProvider { user_data: user_data.clone(), PageRouter {} }
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
