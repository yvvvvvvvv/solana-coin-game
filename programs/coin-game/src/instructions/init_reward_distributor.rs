use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, Transfer};

use crate::errors::ErrorCode;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Ix {
    // bump: u8,
    init_amount: u64,
    player: Pubkey,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: Ix)]
pub struct InitRewardDistributorCtx<'info> {
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
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// impl<'info> InitRewardDistributorCtx<'info> {

//     fn into_transfer_to_pda_context(
//         &self,
//     ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
//         let cpi_accounts = Transfer {
//             from: self.player.to_account_info(),
//             mint: self.mint.to_account_info(),
//             to: self.reward_distributor.to_account_info(),
//             authority: self.player.to_account_info(),
//         };
//         CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
//     }
// }

pub fn handler(ctx: Context<InitRewardDistributorCtx>, ix: Ix) -> Result<()> {
    let reward_distributor = &mut ctx.accounts.reward_distributor;
    // reward_distributor.bump = ix.bump;
    reward_distributor.init_amount = ix.init_amount;
    reward_distributor.player = ix.player;
    reward_distributor.claimed = false;

    // token::transfer(
    //     ctx.accounts.into_transfer_to_pda_context(),
    //     ix.init_amount,
    //     // ctx.accounts.mint.decimals,
    // )?;



    let mut lamports: u64 = ix.init_amount; //* 1_000_000_000; // 25 sol/ 25000 spl //1_000_000_000;//
    lamports = lamports.checked_mul(1_000_000_000).unwrap();
    // sol transfer
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.player.key(),
        &ctx.accounts.reward_distributor.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.player.to_account_info(),
            ctx.accounts.reward_distributor.to_account_info(),
        ],
    )?;

    Ok(())
}
