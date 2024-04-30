// Importing necessary modules from the NEAR SDK
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128, U64};

// Smart Contract declaration with the near_bindgen macro to prepare it for NEAR platform deployment
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]  // Implements serialization, deserialization, and a default panic for uninitialized state
pub struct Roulette {
    owner: AccountId,  // Owner of the contract (uses AccountId for NEAR account addressing)
    parameters: Parameters,  // Game parameters encapsulated within a struct
    game_started: bool,  // Flag to indicate if the game has started
    end_time: u64,  // Timestamp for when the game ends
    total_pot: Balance,  // The total amount of tokens in the pot
    bidders: Vector<AccountId>,  // A vector of bidders (AccountId smart pointers manage memory safely)
    bids: LookupMap<AccountId, Balance>,  // Maps bidders to their bid amounts (efficient memory usage with smart pointers)
}

// Struct to hold game parameters, demonstrating encapsulation and data management
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Parameters {
    game_length: u64,
    max_bet: Balance,
    house_fee: u8,
}

// Implementation block for the Roulette smart contract
#[near_bindgen]
impl Roulette {
    // Initializes a new game with specified parameters
    // Demonstrates error handling with assertions to ensure correct initialization
    #[init]
    pub fn new(game_length: U64, max_bet: U128, house_fee: u8) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner: env::signer_account_id(),
            parameters: Parameters {
                game_length: game_length.into(),
                max_bet: max_bet.into(),
                house_fee,
            },
            game_started: false,
            end_time: 0,
            total_pot: 0,
            bidders: Vector::new(b"b"),
            bids: LookupMap::new(b"m"),
        }
    }

    // Allows the owner to start the game
    // Demonstrates error handling and state management
    pub fn start_game(&mut self) {
        assert!(
            env::signer_account_id() == self.owner,
            "Only the owner can start the game"
        );
        assert!(!self.game_started, "The game has already started");
        self.game_started = true;
        self.end_time = env::block_timestamp() / 1_000_000_000 + self.parameters.game_length;
    }

    // Function for bidders to submit their bids
    // Showcases error handling and smart pointers for memory management
    pub fn submit_bid(&mut self, bid_amount: U128) {
        assert!(self.game_started, "The game has not started");
        assert!(
            env::block_timestamp() / 1_000_000_000 <= self.end_time,
            "The game has ended"
        );
        let bidder = env::signer_account_id();
        let bid = bid_amount.0;
        assert!(
            bid <= self.parameters.max_bet && bid > 0,
            "The bid amount is out of allowed range"
        );
        assert!(
            self.bids.get(&bidder).is_none(),
            "The bidder has already placed a bid"
        );

        self.bidders.push(&bidder);  // Efficient memory operation with smart pointers
        self.bids.insert(&bidder, &bid);  // Key-value storage with memory safety
        self.total_pot += bid;  // Safe state update
    }

    // Ends the game and selects a winner
    // Utilizes Rust's strong type system and error handling
    pub fn end_game(&mut self) {
        assert!(self.game_started, "The game is not running");
        assert!(
            env::block_timestamp() / 1_000_000_000 > self.end_time,
            "The game has not yet ended"
        );

        // Simplified winner selection to demonstrate logic flow and error handling
        let winner_index = env::block_timestamp() as usize % self.bidders.len();
        let winner_id = self.bidders.get(winner_index).unwrap();  // Safe unwrap due to previous checks
        let fee = (self.parameters.house_fee as Balance * self.total_pot) / 100;
        let payout = self.total_pot - fee;  // Safe arithmetic operation

        // State reset demonstrates memory management and clean-up
        self.game_started = false;
        self.total_pot = 0;
        self.bidders.clear();  // Clearing the vector for the next game
        self.bids.clear();  // Clears the bid map

        env::log(format!("Winner: {}. Payout: {}.", winner_id, payout).as_bytes());
    }

    // View method to get the total pot
    // Showcases safe read-only access to state
    pub fn get_total_pot(&self) -> U128 {
        self.total_pot.into()
    }

    // Additional methods as needed for game management and state querying
}