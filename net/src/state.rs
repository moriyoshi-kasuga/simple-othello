#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnState {
    /// authenticating stage
    Login,
    /// in the lobby
    Lobby,
    /// in the game room
    Room,
    /// playing a game
    Game,
}
