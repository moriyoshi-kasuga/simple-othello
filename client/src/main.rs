use std::str::FromStr;

use yew::prelude::*;

fn main() {
    let level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(log::LevelFilter::Info);
    console_log::init_with_level(level).expect("Failed to initialize logger");

    yew::Renderer::<client::App>::new().render();
}
