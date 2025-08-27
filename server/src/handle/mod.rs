use std::net::SocketAddr;

use axum::{body::Bytes, extract::ws::WebSocket};
use net::{
    DecodablePacket,
    packets::{lobby::LobbyRequestPacket, login::LoginRequestPacket, room::RoomRequestPacket},
    state::ConnState,
};
use uid::Uid;

use crate::state::{AppState, connection::Connection, user::User};

pub mod lobby;
pub mod room;

pub async fn handle_socket(addr: SocketAddr, socket: WebSocket, state: AppState) {
    log::info!("New connection from {}", addr);
    let connection = Connection::new(socket);
    macro_rules! close {
        () => {
            connection.close().await;
            return;
        };
    }
    let user = 'l: {
        if connection.get_conn_state() != ConnState::Login {
            log::warn!("Connection from {} attempted to skip login state", addr);
            close!();
        }

        let Some(packet) = connection.receive::<LoginRequestPacket>().await else {
            log::warn!("Connection from {} failed to send login packet", addr);
            close!();
        };
        match packet {
            LoginRequestPacket::Login(req) => {
                // Validate username
                if req.username.is_empty() {
                    log::warn!("Connection from {} attempted to login with empty username", addr);
                    close!();
                }
                if req.username.len() > 32 {
                    log::warn!("Connection from {} attempted to login with username longer than 32 chars", addr);
                    close!();
                }
                log::info!("User '{}' logging in from {}", req.username, addr);
                break 'l User::new(Uid::new(), req.username, connection);
            }
        };
    };

    macro_rules! close {
        () => {
            user.connection.close().await;
            return;
        };
    }

    state.add_user(user.clone()).await;

    log::info!("User '{}' logged in", user.username);

    while let Some::<Bytes>(value) = user.connection.receive_raw().await {
        match user.connection.get_conn_state() {
            ConnState::Login => {
                log::error!("User '{}' is in login state while already logged in", user.username);
                close!();
            } // Should not happen
            ConnState::Lobby => {
                let Some(value) = LobbyRequestPacket::decode(&value) else {
                    log::warn!("User '{}' sent invalid lobby packet", user.username);
                    close!();
                };
                lobby::handle_lobby(&state, &user, value).await
            }
            ConnState::Room => {
                let Some(value) = RoomRequestPacket::decode(&value) else {
                    log::warn!("User '{}' sent invalid room packet", user.username);
                    close!();
                };
                room::handle_room(&state, &user, value).await
            }
            ConnState::Game => {
                log::warn!("User '{}' entered game state (not yet implemented)", user.username);
                // Game state is not yet implemented
                continue;
            }
        }
    }

    log::info!("User '{}' disconnected", user.username);

    state.close_user(user.uid).await;
}
