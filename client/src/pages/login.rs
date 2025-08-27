use dioxus::prelude::*;
use net::packets::UserData;

use crate::state::use_connection;

#[derive(PartialEq, Props, Clone)]
struct Props {
    on_login: EventHandler<UserData>,
}

#[component]
pub fn Login(props: Props) -> Element {
    let connection = use_connection();

    let mut username = use_signal(String::new);

    let on_submit = {
        let username = username.clone();
        let connection = connection.clone();
        let on_login = props.on_login.clone();
        move |e: FormEvent| {
            todo!();
            ()
        }
    };

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-100",
            div { class: "bg-white p-8 rounded shadow-md w-full max-w-md",
                h2 { class: "text-2xl font-bold mb-6 text-center", "Login" }
                form { onsubmit: on_submit,
                    div { class: "mb-4",
                        label { class: "block text-gray-700 text-sm font-bold mb-2", "Username" }
                        input {
                            r#type: "text",
                            class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                            placeholder: "Enter your username",
                            value: "{username}",
                            oninput: move |e| username.set(e.value().clone()),
                            required: true,
                        }
                    }
                    button {
                        r#type: "submit",
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline w-full",
                        "Login"
                    }
                }
            }
        }
    }
}
