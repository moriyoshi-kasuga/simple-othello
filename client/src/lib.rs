use yew::prelude::*;

mod components;
mod pages;

use crate::state::connection::Connection;

pub mod state;

#[function_component(App)]
pub fn app() -> Html {
    let connection = use_state(|| None as Option<Connection>);

    use_effect_with((), {
        let connection = connection.clone();
        move |_| {
            connection.set(Connection::new());

            || ()
        }
    });

    match &*connection {
        Some(conn) => html! {
            <ContextProvider<Connection> context={conn.clone()}>
                <pages::main::MainPage />
            </ContextProvider<Connection>>
        },
        _ => html! { <pages::error::ErrorPage /> },
    }
}
