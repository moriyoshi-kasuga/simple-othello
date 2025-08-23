use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    Router,
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    while let Some(msg) = socket.recv().await {
        let Ok(msg) = msg else {
            // client disconnected
            return;
        };

        // if socket.send(msg).await.is_err() {
        //     // client disconnected
        //     return;
        // }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
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
