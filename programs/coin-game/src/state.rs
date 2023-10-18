use anchor_lang::prelude::*;

pub const FLIP_DEFAULT_SIZE: usize = 1;
pub const FLIP_PREFIX: &str = "state";

#[account]
pub struct CoinFlipState {
    pub game_result: u8
}

// impl CoinFlipState {
// }
