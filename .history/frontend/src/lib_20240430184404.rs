use wasm_bindgen::prelude::*;
use yew::prelude::*;

enum Msg {
    StartGame,
    SubmitBid(u128),
}

struct Model {
    game_started: bool,
    total_pot: u128,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    // Changed to use `Context<Self>` for Yew 0.18+ compatibility
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            game_started: false,
            total_pot: 0,
        }
    }

    // Updated to match the latest Yew signature which includes `&Context<Self>`
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartGame => {
                self.game_started = true;
                true // Indicate that the component should re-render
            }
            Msg::SubmitBid(amount) => {
                self.total_pot += amount;
                true // Indicate that the component should re-render
            }
        }
    }

    // Updated to include `&Context<Self>` in the `view` method
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "NEAR Roulette Game" }</h1>
                if !self.game_started {
                    <button onclick={ctx.link().callback(|_| Msg::StartGame)}>{ "Start Game" }</button>
                } else {
                    // Note: Handling user input correctly will require a bit more logic
                    <input type="number" id="bid_amount" name="bid_amount" />
                    <button onclick={ctx.link().callback(|_| Msg::SubmitBid(100))}>{ "Submit Bid" }</button>
                    <p>{ format!("Total Pot: {}", self.total_pot) }</p>
                }
            </div>
        }
    }
}

// Ensure you're importing wasm_bindgen correctly
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}