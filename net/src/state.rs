use std::sync::atomic::{AtomicU8, Ordering};

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

pub struct AtomicConnState(AtomicU8);

impl AtomicConnState {
    pub const fn new(state: ConnState) -> Self {
        Self(AtomicU8::new(state as u8))
    }

    pub fn load(&self) -> ConnState {
        let raw = self.0.load(Ordering::Acquire);
        unsafe { std::mem::transmute(raw) }
    }

    pub fn store(&self, state: ConnState) {
        self.0.store(state as u8, Ordering::Release);
    }
}
