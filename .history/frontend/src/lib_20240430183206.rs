use yew::prelude::*;

enum Msg {
    StartGame,
    SubmitBid(u128),
}

struct Model {
    link: ComponentLink<Self>,
    game_started: bool,
    total_pot: u128,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            game_started: false,
            total_pot: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                // Placeholder: Logic to start the game
                self.game_started = true;
                true // Rerender
            },
            Msg::SubmitBid(amount) => {
                // Placeholder: Logic to submit a bid
                self.total_pot += amount; // This is just a placeholder
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "NEAR Roulette Game" }</h1>
                if !self.game_started {
                    <button onclick=self.link.callback(|_| Msg::StartGame)>{ "Start Game" }</button>
                } else {
                    <input type="number" id="bid_amount" name="bid_amount" />
                    <button onclick=self.link.callback(|_| Msg::SubmitBid(100))>{ "Submit Bid" }</button>
                    <p>{ format!("Total Pot: {}", self.total_pot) }</p>
                }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}