use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_login: Callback<String>,
}

#[function_component(LoginView)]
pub fn login_view(props: &Props) -> Html {
    let username_ref = use_node_ref();

    let on_login_click = {
        let username_ref = username_ref.clone();
        let on_login = props.on_login.clone();
        Callback::from(move |_| {
            if let Some(input) = username_ref.cast::<web_sys::HtmlInputElement>() {
                let username = input.value();
                if !username.is_empty() {
                    on_login.emit(username);
                }
            }
        })
    };

    html! {
        <div class="login-view">
            <h1>{ "Othello" }</h1>
            <p>{ "Enter your username to start." }</p>
            <input ref={username_ref} type="text" placeholder="Username" />
            <button onclick={on_login_click}>{ "Login" }</button>
        </div>
    }
}
