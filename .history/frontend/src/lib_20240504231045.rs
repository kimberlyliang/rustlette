use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local; // Ensure you're importing spawn_local
use web_sys::HtmlInputElement;
use yew::virtual_dom::AttrValue;

#[derive(Debug)]
enum Msg {
    StartGame,
    SubmitBid,
    UpdateBidAmount(String),
}

#[wasm_bindgen(module = "/static/near_integration.js")]
extern "C" {
    async fn init_contract();
    fn sign_in();
}

#[function_component(Model)]
fn model() -> Html {
    let game_started = use_state(|| false);
    let total_pot = use_state(|| 0);
    let bid_amount = use_state(|| String::new());
    let is_bid_valid = use_state(|| true);

    // Initialize NEAR contract on component mount
    {
        use_effect(|| {
            spawn_local(async {
                init_contract().await.expect("Failed to initialize contract");
            });
            || ()
        });
    }

    let sign_in = Callback::from(|_| {
        sign_in(); 
    });

    // Callback to handle starting the game
    let start_game = {
        let game_started = game_started.clone();
        Callback::from(move |_| game_started.set(true))
    };

    // Callback to handle input changes in the bid amount field
    let update_bid_amount = {
        let bid_amount = bid_amount.clone();
        let is_bid_valid = is_bid_valid.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value();

            // Validate the input as a positive u128 number
            let is_valid = input_value.parse::<u128>().is_ok() && input_value.parse::<u128>().unwrap() > 0;

            // Update the bid amount and validation state
            bid_amount.set(input_value);
            is_bid_valid.set(is_valid);
        })
    };

    // Callback to handle form submission
    let submit_bid = {
        let bid_amount = bid_amount.clone();
        let total_pot = total_pot.clone();
        let is_bid_valid = *is_bid_valid;

        Callback::from(move |_| {
            if is_bid_valid {
                if let Ok(amount) = bid_amount.parse::<u128>() {
                    total_pot.set(*total_pot + amount);
                }
            }
        })
    };

    html! {
        <div>
            <h1>{ "NEAR Roulette Game" }</h1>
            <button onclick={sign_in}>{ "Sign in" }</button>
            if !*game_started {
                <button onclick={start_game}>{ "Start Game" }</button>
            } else {
                <div>
                    <input type="number"
                           id="bid_amount"
                           name="bid_amount"
                           placeholder="Enter your bid"
                           value={AttrValue::from(bid_amount.to_string())}
                           oninput={update_bid_amount}
                           style={if *is_bid_valid { "" } else { "border: 2px solid red;" }}
                    />
                    <button onclick={submit_bid} disabled={!*is_bid_valid}>{ "Submit Bid" }</button>
                    <p>{ format!("Total Pot: {}", *total_pot) }</p>
                </div>
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}