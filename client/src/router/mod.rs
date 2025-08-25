use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{lobby::Lobby, login::Login, room::Room};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/lobby")]
    Lobby,
    #[at("/room/:key")]
    Room { key: String },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Lobby => html! { <Lobby /> },
        Route::Room { key } => html! { <Room room_key={key} /> },
    }
}
