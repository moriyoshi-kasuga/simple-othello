use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod contexts;
pub mod hooks;
pub mod router;
pub mod services;

use contexts::AppContextProvider;
use router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppContextProvider>
            <div class="container">
                <h1>{ "Simple Othello" }</h1>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </AppContextProvider>
    }
}
