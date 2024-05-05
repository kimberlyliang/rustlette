use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use web_sys::HtmlInputElement;

#[derive(Debug)]
enum Msg {
    StartGame,
    EndGame,
    SubmitBid,
    UpdateBidAmount(String),
    UpdateTimer,
}

#[wasm_bindgen(module = "/static/near_integration.js")]
extern "C" {
    async fn initContract();
    fn signIn();
}

#[wasm_bindgen(module = "/static/near_integration.js")]
extern "C" {
    async fn initContract();
    fn signIn();
}

#[function_component(Model)]
fn model() -> Html {
    // State hooks
    let game_started = use_state(|| false);
    let total_pot = use_state(|| 0);
    let bid_amount = use_state(|| String::new());
    let is_bid_valid = use_state(|| true);

    // Initialize NEAR contract on component mount
    {
        use_effect_with_deps(|_| {
            spawn_local(async {
                initContract().await;
            });
            || ()
        }, ());
    }

    let sign_in = Callback::from(|_| {
        signIn();
    });
    let timer = use_state(|| 0);
    let interval = use_state(|| None as Option<Interval>);

    // Callback to handle starting the game
    let start_game = {
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();

        Callback::from(move |_| {
            // Clone variables to avoid moving them out of the closure
            let timer_clone = timer.clone();
            let interval_clone = interval.clone();

            // Start the game
            game_started.set(true);
            timer_clone.set(0);

            // Create a new interval timer
            let new_interval = Interval::new(1000, move || {
                timer_clone.set(*timer_clone + 1);
            });

            // Insert the new interval into the state
            interval.insert(new_interval);
        })
    };

    // Callback to handle ending the game
    let end_game = {
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();

        Callback::from(move |_| {
            // Reset the game state
            game_started.set(false);
            timer.set(0);

            // Take the interval from the state and cancel it if it exists
            if let Some(interval_instance) = interval.take() {
                interval_instance.cancel();
            }
        })
    };

    // Callback to handle input changes in the bid amount field
    let update_bid_amount = {
        let bid_amount = bid_amount.clone();
        let is_bid_valid = is_bid_valid.clone();

        Callback::from(move |e: InputEvent| {
            let input: Result<HtmlInputElement, _> = e.target().unwrap().dyn_into();
            if let Ok(input) = input {
                let input_value = input.value();
                let is_valid = input_value.parse::<u128>().is_ok() && input_value.parse::<u128>().unwrap() > 0;
                bid_amount.set(input_value);
                is_bid_valid.set(is_valid);
            }
        })
    };

    // Callback to handle form submission
    let submit_bid = {
        let bid_amount = bid_amount.clone();
        let total_pot = total_pot.clone();
        let is_bid_valid = *is_bid_valid;

        Callback::from(move |_| {
            // Clone variables and check validity before submitting bid
            if is_bid_valid {
                if let Ok(amount) = bid_amount.parse::<u128>() {
                    total_pot.set(*total_pot + amount);
                }
            }
        })
    };

    // Rendering logic
    html! {
        <div>
            <h1>{ "NEAR Roulette Game" }</h1>
            <button onclick={sign_in}>{ "Sign in" }</button>
            if !*game_started {
                <button onclick={start_game}>{ "Start Game" }</button>
            } else {
                <div>
                    <input
                        type="number"
                        id="bid_amount"
                        name="bid_amount"
                        placeholder="Enter your bid"
                        value={AttrValue::from(bid_amount.to_string())}
                        oninput={update_bid_amount}
                        style={if *is_bid_valid { "" } else { "border: 2px solid red;" }}
                    />
                    <button onclick={submit_bid} disabled={!*is_bid_valid}>{ "Submit Bid" }</button>
                    <p>{ format!("Total Pot: {}", *total_pot) }</p>
                    <p>{ format!("Time: {} seconds", *timer) }</p>
                    <button onclick={end_game}>{ "End Game" }</button>
                </div>
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}