use gloo::timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum Msg {
    StartGame,
    EndGame,
    SubmitBid,
    UpdateBidAmount(String),
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
    let timer = use_state(|| 0);
    let interval = use_state(|| None as Option<Rc<RefCell<Interval>>>);

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

    // Callback to handle starting the game
    let start_game = {
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();

        Callback::from(move |_| {
            game_started.set(true);
            timer.set(0);

            // Stop and remove existing interval if it exists
            // if let Some(existing_interval) = &*interval {
            //     existing_interval.borrow_mut().cancel();
            // }

            // Create a new interval that updates the timer state every second
            let new_interval = Interval::new(1000, {
                let timer = timer.clone();
                move || {
                    timer.set(*timer + 1);
                }
            });

            // Store the new interval
            interval.set(Some(Rc::new(RefCell::new(new_interval))));
        })
    };

    // Callback to handle ending the game
    let end_game = {
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();

        Callback::from(move |_| {
            game_started.set(false);
            timer.set(0);

            // Stop and remove existing interval if it exists
            // if let Some(existing_interval) = &*interval {
            //     existing_interval.borrow_mut().cancel();
            // }
            
            // Reset the interval state
            interval.set(None);
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
                    <input type="number"
                           id="bid_amount"
                           name="bid_amount"
                           placeholder="Enter your bid"
                           value={bid_amount.to_string()}
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
