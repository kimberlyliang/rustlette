use gloo::timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

// Custom error types
#[derive(Debug, Clone)]
enum AppError {
    NetworkError(String),
    SigninError(String),
    StartGameError(String),
    BidError(String),
    GeneralError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::SigninError(msg) => write!(f, "Sign-in error: {}", msg),
            AppError::StartGameError(msg) => write!(f, "Start game error: {}", msg),
            AppError::BidError(msg) => write!(f, "Bid error: {}", msg),
            AppError::GeneralError(msg) => write!(f, "General error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Message enum for the Model component
enum Msg {
    StartGame,
    EndGame,
    SubmitBid,
    UpdateBidAmount(String),
    HandleError(AppError),
}

#[wasm_bindgen(module = "/static/near_integration.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn initContract() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    fn signIn() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    async fn startGame() -> Result<(), JsValue>;
}

#[function_component(Model)]
fn model() -> Html {
    // State handles
    let game_started = use_state(|| false);
    let total_pot = use_state(|| 0);
    let bid_amount = use_state(|| String::new());
    let is_bid_valid = use_state(|| true);
    let timer = use_state(|| 0);
    let interval = use_state(|| None as Option<Rc<RefCell<Interval>>>);
    let error = use_state(|| None::<AppError>);

    // Initialize NEAR contract on component mount
    {
        let error_clone = error.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match initContract().await {
                    Ok(_) => {},
                    Err(e) => {
                        let app_error = AppError::NetworkError(
                            e.as_string().unwrap_or_else(|| "Failed to initialize contract".into())
                        );
                        error_clone.set(Some(app_error));
                    }
                }
            });
            || ()
        }, ());
    }

    // Sign-in callback
    let sign_in = Callback::from({
        let error_clone = error.clone();
        move |_| {
            match signIn() {
                Ok(_) => {},
                Err(e) => {
                    let app_error = AppError::SigninError(
                        e.as_string().unwrap_or_else(|| "Sign-in failed".into())
                    );
                    error_clone.set(Some(app_error));
                }
            }
        }
    });

    // Start game
    let start_game = Callback::from({
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();
        let error_clone = error.clone();

        move |_| {
            game_started.set(true);
            timer.set(0);

            // Create a new interval
            let new_interval = Interval::new(1000, {
                let timer_clone = timer.clone();
                move || {
                    timer_clone.set(*timer_clone + 1);
                }
            });

            interval.set(Some(Rc::new(RefCell::new(new_interval))));

            spawn_local(async move {
                match startGame().await {
                    Ok(_) => {},
                    Err(e) => {
                        let app_error = AppError::StartGameError(
                            e.as_string().unwrap_or_else(|| "Failed to start game".into())
                        );
                    }
                }
            });
        }
    });

    // End game 
    let end_game = Callback::from({
        let game_started = game_started.clone();
        let timer = timer.clone();
        let interval = interval.clone();

        move |_| {
            game_started.set(false);
            timer.set(0);

            // Reset interval state
            // if let Some(existing_interval) = interval.as_ref() {
            //     existing_interval.borrow_mut().cancel();
            // }

            interval.set(None);
        }
    });

    // Update bid amount 
    let update_bid_amount = Callback::from({
        let bid_amount = bid_amount.clone();
        let is_bid_valid = is_bid_valid.clone();
        let error_clone = error.clone();

        move |e: InputEvent| {
            let input: Result<HtmlInputElement, _> = e.target().unwrap().dyn_into();
            match input {
                Ok(input_element) => {
                    let input_value = input_element.value();
                    match input_value.parse::<u128>() {
                        Ok(amount) => {
                            bid_amount.set(input_value);
                            is_bid_valid.set(amount > 0);
                        },
                        Err(_) => {
                            let app_error = AppError::BidError(
                                "Invalid bid amount. It must be a positive number.".into()
                            );
                            error_clone.set(Some(app_error));
                            is_bid_valid.set(false);
                        }
                    }
                },
                Err(_) => {
                    let app_error = AppError::GeneralError("Failed to retrieve input element.".into());
                    error_clone.set(Some(app_error));
                }
            }
        }
    });

    // Submit bid callback
    let submit_bid = Callback::from({
        let bid_amount = bid_amount.clone();
        let total_pot = total_pot.clone();
        let is_bid_valid = *is_bid_valid;
        let error_clone = error.clone();

        move |_| {
            if is_bid_valid {
                match bid_amount.parse::<u128>() {
                    Ok(amount) => total_pot.set(*total_pot + amount),
                    Err(_) => {
                        let app_error = AppError::BidError(
                            "Failed to parse bid amount for submission.".into()
                        );
                        error_clone.set(Some(app_error));
                    }
                }
            }
        }
    });

    // Render error function
    let render_error = |error: &AppError| {
        html! { <p class="error" style="color: red; font-weight: bold; margin-bottom: 10px;">{ error.to_string() }</p> }
    };

    // page
    html! {
        <div style="max-width: 800px; margin: auto; padding: 20px; background-color: white; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); border-radius: 8px;">
            <h1 style="text-align: center; color: #333; margin-bottom: 20px;">{ "NEAR Roulette Game" }</h1>

            if let Some(ref err) = *error {
                { render_error(err) }
            }

            // Center the buttons and add margin for separation
            <div style="display: flex; justify-content: center; gap: 20px; margin-bottom: 20px;">
                <button onclick={sign_in.clone()} style="background-color: #007BFF; color: white; border: none; border-radius: 4px; padding: 10px 15px; cursor: pointer; transition: background-color 0.3s;">{ "Sign in" }</button>
                if !*game_started {
                    <button onclick={start_game.clone()} style="background-color: #007BFF; color: white; border: none; border-radius: 4px; padding: 10px 15px; cursor: pointer; transition: background-color 0.3s;">{ "Start Game" }</button>
                }
            </div>

            if *game_started {
                <div style="margin-top: 20px;">
                    <input type="number"
                           id="bid_amount"
                           name="bid_amount"
                           placeholder="Enter your bid"
                           value={bid_amount.to_string()}
                           oninput={update_bid_amount}
                           style={format!("width: 100%; padding: 8px; border: 1px solid {}; border-radius: 4px; margin-bottom: 10px;",
                                          if *is_bid_valid { "#ccc" } else { "red" })}
                    />
                    <button onclick={submit_bid} disabled={!*is_bid_valid} style="background-color: #007BFF; color: white; border: none; border-radius: 4px; padding: 10px 15px; cursor: pointer; transition: background-color 0.3s;">{ "Submit Bid" }</button>
                    <p style="font-weight: bold; margin-bottom: 5px;">{ format!("Total Pot: {}", *total_pot) }</p>
                    <p style="font-weight: bold; margin-bottom: 10px;">{ format!("Time: {} seconds", *timer) }</p>
                    <button onclick={end_game} style="background-color: #007BFF; color: white; border: none; border-radius: 4px; padding: 10px 15px
                    ; cursor: pointer; transition: background-color 0.3s;">{ "End Game" }</button>
                    </div>
                }
            </div>
        }
    }
    
    #[wasm_bindgen(start)]
    pub fn run_app() {
        yew::start_app::<Model>();
    }
    