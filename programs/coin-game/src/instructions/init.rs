use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

use crate::errors::ErrorCode;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Ix {
    // bump: u8,
    side: u8,
    init_amount: u64,
    player: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: Ix)]
pub struct InitCtx<'info> {
    #[account(
        init,
        payer = player,
        space = REWARD_DEFULT_SIZE,
        seeds = [REWARD_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
    )]
    pub reward_distributor: Box<Account<'info, RewardDistributor>>,

    pub mint: Account<'info, Mint>,

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

    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitCtx>, ix: Ix) -> Result<()> {
    let reward_distributor = &mut ctx.accounts.reward_distributor;
    // reward_distributor.bump = ix.bump;
    reward_distributor.init_amount = ix.init_amount;
    reward_distributor.player = ix.player;
    reward_distributor.claimed = false;

    // Check if the side is valid (true for heads, false for tails).
    if ix.side != 1 && ix.side != 2 {
        return Err(ErrorCode::InvalidSide.into());
    };

    // Generate a random number (for simplicity, you can use a more secure method in production).
    let random_number = anchor_lang::solana_program::sysvar::clock::Clock::get()?.unix_timestamp as u8;

    // Determine the result (1 for heads, 2 for tails).
    let win_or_lose = random_number % 2 == ix.side - 1;

    if win_or_lose {
        reward_distributor.reward = reward_distributor.reward + (ix.init_amount * 2);
        ctx.accounts.coin_flip_state.game_result = true;
    } else {
        ctx.accounts.coin_flip_state.game_result = false;
    }

    let mut lamports: u64 = ix.init_amount; //* 1_000_000_000; // 25 sol/ 25000 spl //1_000_000_000;//
    lamports = lamports.checked_mul(1_000_000_000).unwrap();
    // sol transfer
    let ixx = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.player.key(),
        &ctx.accounts.reward_distributor.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ixx,
        &[
            ctx.accounts.player.to_account_info(),
            ctx.accounts.reward_distributor.to_account_info(),
        ],
    )?;

    Ok(())
}
