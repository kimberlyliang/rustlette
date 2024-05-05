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

    pub fn start_game(&mut self) {
        assert!(env::signer_account_id() == self.owner, "Only the owner can start the game");
        assert!(!self.game_started, "The game has already started");
        self.game_started = true;
        self.end_time = env::block_timestamp() / 1_000_000_000 + self.parameters.game_length;
    }

    pub fn submit_bid(&mut self, bid_amount: U128) {
        assert!(self.game_started, "The game has not started");
        assert!(env::block_timestamp() / 1_000_000_000 <= self.end_time, "The game has ended");
        let bidder = env::signer_account_id();
        let bid = bid_amount.0;
        assert!(bid <= self.parameters.max_bet && bid > 0, "The bid amount is out of allowed range");
        assert!(self.bids.get(&bidder).is_none(), "The bidder has already placed a bid");

        self.bidders.push(&bidder);
        self.bids.insert(&bidder, &bid);
        self.total_pot += bid;
    }

    pub fn end_game(&mut self) {
        assert!(self.game_started, "The game is not running");
        assert!(
            env::block_timestamp() / 1_000_000_000 > self.end_time,
            "The game has not yet ended"
        );
    
        let winner_index = env::block_timestamp() as usize % self.bidders.len();
        let winner_id = self.bidders.get(winner_index).unwrap();  // Safe unwrap due to previous checks
        let fee = (self.parameters.house_fee as Balance * self.total_pot) / 100;
        let payout = self.total_pot - fee;
    
        // Reset game state
        self.game_started = false;
        self.total_pot = 0;
        self.bidders.clear();  // Clearing the vector for the next game
        self.bids = LookupMap::new(b"m");  // Reinitialize the bids map
    
        env::log(format!("Winner: {}. Payout: {}.", winner_id, payout).as_bytes());
    }
    

    pub fn get_total_pot(&self) -> U128 {
        self.total_pot.into()
    }
}
