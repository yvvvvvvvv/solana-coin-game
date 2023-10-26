use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FlipIx {
    side: u8,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: FlipIx)]
pub struct FlipCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init,
        seeds = [FLIP_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
        payer = player,
        space = FLIP_DEFAULT_SIZE
    )]
    pub coin_flip_state: Box<Account<'info, CoinFlipState>>,

    #[account(
        init,
        payer = player,
        space = REWARD_DEFULT_SIZE, // 設定合適的大小
        seeds = [REWARD_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
    )]
    pub reward_distributor: Box<Account<'info, RewardDistributor>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FlipCtx>, ix: FlipIx) -> Result<()> {
    // Check if the side is valid (true for heads, false for tails).
    if ix.side != 1 && ix.side != 2 {
        return Err(ErrorCode::InvalidSide.into());
    };

    // Generate a random number (for simplicity, you can use a more secure method in production).
    let random_number = anchor_lang::solana_program::sysvar::clock::Clock::get()?.unix_timestamp as u8;

    // if random_number % 2 == 0 {
    //     // println!("Result is head, user {}!!!🎉", win_or_lose);
    //     ctx.accounts.coin_flip_state.game_result = 1;
    // } else {
    //     // println!("Result is tail, user {}...😏", win_or_lose);
    //     ctx.accounts.coin_flip_state.game_result = 2;
    // };

    // Determine the result (1 for heads, 2 for tails).
    let win_or_lose = random_number % 2 == ix.side - 1;

    let mut reward_distributor = &mut ctx.accounts.reward_distributor;
    let init_amount = reward_distributor.init_amount;

    if win_or_lose {
        reward_distributor.reward = reward_distributor.reward + (init_amount * 2);
        ctx.accounts.coin_flip_state.game_result = true;
    } else {
        ctx.accounts.coin_flip_state.game_result = false;
    }

    Ok(())
}

