use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod coin_game {
    use super::*;

    pub fn play(ctx: Context<InitCtx>, ix: FlipIx) -> Result<()> {
        instructions::play::handler(ctx, ix)
    }
}


