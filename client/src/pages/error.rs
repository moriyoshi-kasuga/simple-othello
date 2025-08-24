use yew::prelude::*;

#[function_component(ErrorPage)]
pub fn error_page() -> Html {
    html! {
        <div>
            <h1>{ "Connection Error" }</h1>
            <p>{ "There was an error with the WebSocket connection. Please try refreshing the page." }</p>
        </div>
    }
}
