use anchor_lang::prelude::*;

pub const FLIP_DEFAULT_SIZE: usize = 1 + 24 + 32 + 32 + 32 + 8 + 8 + 1 + 32;
pub const FLIP_PREFIX: &str = "state";

pub const REWARD_DEFULT_SIZE: usize = 1 + 32 + 8 + 32 + 1 + 32 + 8;
pub const REWARD_PREFIX: &str = "reward_state";

#[account]
pub struct CoinFlipState {
    pub game_result: bool
}

#[account]
pub struct RewardDistributor {
    // pub bump: u8,
    pub coin_game: Pubkey,
    pub init_amount: u64,
    pub player: Pubkey,
    pub claimed: bool,
    pub reward: u64
}

// impl CoinFlipState {
// }
