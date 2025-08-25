use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div>
            <h2>{"Login"}</h2>
            <div class="form-group">
                <label for="username">{"Username"}</label>
                <input type="text" id="username" placeholder="Enter your username" />
            </div>
            <button>{"Login"}</button>
        </div>
    }
}
