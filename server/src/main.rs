use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use net::models::login::{LoginRequest, LoginResponse};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    println!("New WebSocket connection established");
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if let Ok(login_req) = serde_json::from_str::<LoginRequest>(&text) {
                println!("Received login request: {:?}", login_req);

                let login_res = LoginResponse {
                    token: format!("token-for-{}", login_req.username),
                };

                let json_res = serde_json::to_string(&login_res).unwrap();

                if socket.send(Message::text(json_res)).await.is_err() {
                    println!("Failed to send message");
                    break;
                }
            } else {
                println!("Received something else: {}", text);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let level_filter = std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(log::LevelFilter::Info);
    env_logger::builder().filter_level(level_filter).init();
    log::info!("Starting server...");

    let state = AppState {};
    let app = Router::new().route("/ws", any(handler)).with_state(state);

    let socket_addr: SocketAddr = std::env::var("TCP_LISTENER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            3000,
        ));

    log::info!("Listening on {}", socket_addr);

    let listener = TcpListener::bind(socket_addr)
        .await
        .unwrap_or_else(|_| panic!("TCP listener cannot bind"));

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|_| panic!("Cannot start server"));
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Ctrl+C received, shutting down");
        },
        _ = terminate => {
            log::info!("Terminate signal received, shutting down");
        },
    }
}
