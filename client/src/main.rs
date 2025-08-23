use yew::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    yew::Renderer::<client::App>::new().render();
}
