use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room_key: String,
}

#[function_component(Room)]
pub fn room(props: &RoomProps) -> Html {
    html! {
        <div>
            <h2>{format!("Room: {}", props.room_key)}</h2>

            <div class="button-group">
                <button>{"Select Black"}</button>
                <button>{"Select White"}</button>
            </div>

            <div class="board-container">
                <h3>{"Game Board"}</h3>
                <div class="board">
                    {(0..64).map(|_| html!{ <div class="cell"></div> }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}
