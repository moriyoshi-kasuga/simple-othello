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

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let connection = Connection::new(socket);
    macro_rules! close {
        ($expr:expr) => {
            if $expr {
                connection.close().await;
                return;
            }
        };
        () => {
            connection.close().await;
            return;
        };
    }
    let user = loop {
        close!(connection.get_conn_state() != ConnState::Login);

        let Some(packet) = connection.receive::<LoginRequestPacket>().await else {
            close!();
        };
        match packet {
            LoginRequestPacket::Login(req) => {
                break User::new(Uid::new(), req.username, connection);
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
                close!();
            } // Should not happen
            ConnState::Lobby => {
                let Some(value) = LobbyRequestPacket::decode(&value) else {
                    close!();
                };
                lobby::handle_lobby(&state, &user, value).await
            }
            ConnState::Room => {
                let Some(value) = RoomRequestPacket::decode(&value) else {
                    close!();
                };
                room::handle_room(&state, &user, value).await
            }
            ConnState::Game => todo!(),
        }
    }

    log::info!("User '{}' disconnected", user.username);
}
