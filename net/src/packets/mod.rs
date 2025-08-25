use uid::Uid;

pub mod game;
pub mod lobby;
pub mod login;
pub mod room;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserData {
    pub uid: Uid,
    pub username: String,
}
