use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::ErrorCode;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimIx {
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: ClaimIx)]
pub struct ClaimCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut
        // init,
        // seeds = [FLIP_PREFIX.as_bytes(), ix.identifier.as_ref()],
        // bump,
        // payer = player,
        // space = FLIP_DEFAULT_SIZE
    )]
    pub coin_flip_state: Box<Account<'info, CoinFlipState>>,

    #[account(
        mut
        // init,
        // payer = player,
        // space = REWARD_DEFULT_SIZE, // 設定合適的大小
        // seeds = [REWARD_PREFIX.as_bytes(), ix.identifier.as_ref()],
        // bump,
    )]
    pub reward_distributor: Box<Account<'info, RewardDistributor>>,

    #[account(
        mut, 
        constraint = reward_distributor_token_account.owner == reward_distributor.key() 
        // && 
        // reward_distributor_token_account.mint == reward_distributor.reward_mint @ ErrorCode::InvalidRewardDistributorTokenAccount
    )]
    reward_distributor_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<ClaimCtx>, ix: ClaimIx) -> Result<()> {
    // let reward_distributor = &mut ctx.accounts.reward_distributor;

    // let identifier_seed = ix.identifier.to_le_bytes();
    // let reward_distributor_seed = &[
    //     REWARD_DISTRIBUTOR_SEED.as_bytes(),
    //     ctx.accounts.reward_distributor.coin_game.as_ref(),
    //     identifier_seed.as_ref(),
    //     &[ctx.accounts.reward_distributor.bump],
    // ];
    // let reward_distributor_signer = &[&reward_distributor_seed[..]];

    let mut lamports: u64 = ctx.accounts.reward_distributor.reward; //* 1_000_000_000; // 25 sol/ 25000 spl //1_000_000_000;//
    
    if lamports == 0 {
        return Err(ErrorCode::InvalidClaimAmount.into());
    }
    lamports = lamports.checked_mul(1_000_000_000).unwrap();
    // sol transfer
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.reward_distributor_token_account.key(),
        &ctx.accounts.player.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.reward_distributor_token_account.to_account_info(),
            ctx.accounts.player.to_account_info(),
        ]
        // reward_distributor_signer
    )?;

    ctx.accounts.reward_distributor.reward = 0;

    Ok(())
}
