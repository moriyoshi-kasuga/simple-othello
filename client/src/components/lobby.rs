use yew::prelude::*;

#[function_component(Lobby)]
pub fn lobby() -> Html {
    html! {
        <div>
            <h2>{"Lobby"}</h2>
            <div class="form-group">
                <label for="room_key">{"Room Key"}</label>
                <input type="text" id="room_key" placeholder="Enter room key" />
            </div>
            <div class="button-group">
                <button>{"Create Room"}</button>
                <button>{"Join Room"}</button>
            </div>
        </div>
    }
}
