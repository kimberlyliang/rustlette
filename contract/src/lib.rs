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
    
        let winner_index = (env::block_timestamp() as usize % self.bidders.len() as usize) as u64;
        let winner_id = self.bidders.get(winner_index).unwrap();  // Safe unwrap due to previous checks
        let fee = (self.parameters.house_fee as Balance * self.total_pot) / 100;
        let payout = self.total_pot - fee;
    
        // Reset game state
        self.game_started = false;
        self.total_pot = 0;
        self.bidders.clear();  // Clearing the vector for the next game
        self.bids = LookupMap::new(b"m");  // Reinitialize the bids map
    
        env::log_str(&format!("Winner: {}. Payout: {}.", winner_id, payout));
    }
    

    pub fn get_total_pot(&self) -> U128 {
        self.total_pot.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};
    use near_sdk::json_types::{U128, U64};

    // Helper function to set up the environment and return the contract
    fn setup_contract() -> Roulette {
        let mut context = VMContextBuilder::new();
        context.current_account_id(accounts(0))
            .signer_account_id(accounts(1))
            .account_balance(10u128.pow(26)); // 100 NEAR
        testing_env!(context.build());

        Roulette::new(U64(300), U128(1_000_000_000), 5) // game_length: 300 seconds, max_bet: 1 NEAR, house_fee: 5%
    }

    #[test]
    fn test_initialization() {
        let contract = setup_contract();
        assert_eq!(contract.owner, accounts(1));
        assert_eq!(contract.parameters.max_bet.0, 1_000_000_000);
        assert_eq!(contract.parameters.house_fee, 5);
        assert!(!contract.game_started);
    }

    #[test]
    #[should_panic(expected = "The game has already started")]
    fn test_start_game_twice() {
        let mut contract = setup_contract();
        contract.start_game();
        contract.start_game(); // This should panic
    }

    #[test]
    fn test_game_flow() {
        let mut contract = setup_contract();
        contract.start_game();
        assert!(contract.game_started);

        // Player places a bid
        testing_env!(VMContextBuilder::new()
            .signer_account_id(accounts(2))
            .predecessor_account_id(accounts(2))
            .build());
        contract.submit_bid(U128(500_000_000)); // 0.5 NEAR

        // End the game
        testing_env!(VMContextBuilder::new()
            .block_timestamp(contract.end_time + 1_000_000_000) // Fast forward time past end_time
            .build());
        contract.end_game();

        assert!(!contract.game_started);
        assert_eq!(contract.total_pot, 0);
    }

    #[test]
    #[should_panic(expected = "The bid amount is out of allowed range")]
    fn test_bid_too_high() {
        let mut contract = setup_contract();
        contract.start_game();
        contract.submit_bid(U128(2_000_000_000)); // 2 NEAR, should panic because max_bet is 1 NEAR
    }
}
