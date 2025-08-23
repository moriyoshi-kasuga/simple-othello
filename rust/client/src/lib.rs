use std::sync::Arc;

use gloo_net::websocket::{Message, WebSocketError, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use futures::{SinkExt, StreamExt, lock::Mutex, stream::SplitSink};
use net::models::login::LoginRequest;

const WEBSOCKET_URL: &str = "ws://127.0.0.1:3000/ws";

#[function_component(App)]
pub fn app() -> Html {
    let username_ref = use_node_ref();
    let messages = use_state(Vec::new);
    let ws_tx = use_state(|| None::<Arc<Mutex<SplitSink<WebSocket, Message>>>>);

    let on_login_click = {
        let username_ref = username_ref.clone();
        let ws_tx = ws_tx.clone();
        Callback::from(move |_| {
            if let Some(input) = username_ref.cast::<web_sys::HtmlInputElement>() {
                let username = input.value();
                if !username.is_empty()
                    && let Some(tx) = (*ws_tx).clone()
                {
                    spawn_local(async move {
                        let login_req = LoginRequest { username };
                        let json_req = serde_json::to_string(&login_req).unwrap();
                        if let Err(e) = tx.lock().await.send(Message::Text(json_req)).await {
                            log::error!("Failed to send message: {:?}", e);
                        }
                    });
                }
            }
        })
    };

    let messages_for_effect = messages.clone();
    use_effect_with((), move |_| {
        let ws = WebSocket::open(WEBSOCKET_URL).unwrap();
        let (write, mut read) = ws.split();

        ws_tx.set(Some(Arc::new(Mutex::new(write))));

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        let mut new_messages = (*messages_for_effect).clone();
                        new_messages.push(format!("Received: {}", data));
                        messages_for_effect.set(new_messages);
                    }
                    Ok(Message::Bytes(b)) => {
                        log::info!("Received binary data: {:?}", b);
                    }
                    Err(e) => match e {
                        WebSocketError::ConnectionError => {
                            log::error!("Connection Error");
                        }
                        WebSocketError::ConnectionClose(e) => {
                            log::error!("Connection Close: {:?}", e);
                        }
                        WebSocketError::MessageSendError(e) => {
                            log::error!("Message Send Error: {:?}", e.to_string());
                        }
                        _ => {
                            log::error!("WebSocket Error");
                        }
                    },
                }
            }
            log::info!("WebSocket connection closed");
        });

        || ()
    });

    html! {
        <div>
            <h1>{ "Othello Game" }</h1>
            <div>
                <input ref={username_ref} type="text" placeholder="Enter username" />
                <button onclick={on_login_click}>{ "Login" }</button>
            </div>
            <h2>{ "Received Messages:" }</h2>
            <ul>
                { for messages.iter().map(|msg| html!{ <li>{ msg }</li> }) }
            </ul>
        </div>
    }
}
