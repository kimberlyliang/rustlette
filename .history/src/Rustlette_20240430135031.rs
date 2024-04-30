use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128, U64};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Roulette {
    owner: AccountId,
    parameters: Parameters,
    game_started: bool,
    end_time: u64,
    total_pot: Balance,
    bidders: Vector<AccountId>,
    bids: LookupMap<AccountId, Balance>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Parameters {
    game_length: u64,
    max_bet: Balance,
    house_fee: u8,
}

#[near_bindgen]
impl Roulette {
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
    pub fn start_game(&mut self) {
        assert!(
            env::signer_account_id() == self.owner,
            "Only the owner can start the game"
        );
        assert!(!self.game_started, "The game has already started");
        self.game_started = true;
        self.end_time = env::block_timestamp() / 1_000_000_000 + self.parameters.game_length;
    }

    // Submit a bid to the game
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

        // Here, you would typically call a method on a token contract to transfer tokens
        // from the bidder to this contract. This requires handling promises and callbacks
        // which are not covered in this simple example.

        self.bidders.push(&bidder);
        self.bids.insert(&bidder, &bid);
        self.total_pot += bid;
    }

    // Ends the game, chooses a winner, and resets game state
    pub fn end_game(&mut self) {
        assert!(self.game_started, "The game is not running");
        assert!(
            env::block_timestamp() / 1_000_000_000 > self.end_time,
            "The game has not yet ended"
        );

        // Choose winner logic (simplified)
        // In a real scenario, you'd want a more fair and unpredictable selection
        let winner_index = env::block_timestamp() as usize % self.bidders.len();
        let winner_id = self.bidders.get(winner_index).unwrap();
        // Calculate the payout and house fee
        let fee = (self.parameters.house_fee as Balance * self.total_pot) / 100;
        let payout = self.total_pot - fee;

        // Transfer the payout to the winner and fee to the owner
        // This would involve calling a NEP-141 compliant token contract

        // Reset the game state
        self.game_started = false;
        self.total_pot = 0;
        self.bidders.clear();
        self.bids.clear();

        env::log(format!("Winner: {}. Payout: {}.", winner_id, payout).as_bytes());
    }

    // View methods for contract state

    pub fn get_total_pot(&self) -> U128 {
        self.total_pot.into()
    }

    // Additional methods as needed
}