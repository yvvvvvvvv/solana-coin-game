use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod coin_game {
    use super::*;

    pub fn init(ctx: Context<InitRewardDistributorCtx>, ix: Ix) -> Result<()> {
        instructions::init_reward_distributor::handler(ctx, ix)
    }

    pub fn play(ctx: Context<FlipCtx>, ix: FlipIx) -> Result<()> {
        instructions::play::handler(ctx, ix)
    }
}


