use yew::prelude::*;

use crate::state::connection::Connection;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let conn = use_context::<Connection>().expect("No Connection context found");

    html! {
        <div>
            <h1>{ "Welcome to the Othello Game!" }</h1>
            <p>{ "This is the main page of the Othello game application." }</p>
        </div>
    }
}
