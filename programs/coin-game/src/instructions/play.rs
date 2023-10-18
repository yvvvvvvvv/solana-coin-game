use anchor_lang::prelude::*;
use rand::prelude::*;

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FlipIx {
    side: u8,
    identifier: String,
}

#[derive(Accounts)]
#[instruction(ix: FlipIx)]
pub struct InitCtx<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        init,
        seeds = [FLIP_PREFIX.as_bytes(), ix.identifier.as_ref()],
        bump,
        payer = authority,
        space = FLIP_DEFAULT_SIZE
    )]
    coin_flip_state: Box<Account<'info, CoinFlipState>>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitCtx>, ix: FlipIx) -> Result<()> {
    // Check if the side is valid (true for heads, false for tails).
    if ix.side != 1 && ix.side != 2 {
        return Err(ErrorCode::InvalidSide.into());
    };

    if ix.side == 1 {
        println!("player choose head");
    } else {
        println!("player choose tail");
    };

    // Generate a random number (for simplicity, you can use a more secure method in production).
    let random_number = anchor_lang::solana_program::sysvar::clock::Clock::get()?.unix_timestamp as u8;

    // Determine the result (1 for heads, 2 for tails).
    let win_or_lose = match random_number % 2 == ix.side - 1 {
        true => String::from("WIN"),
        false => String::from("LOSE"),
    };

    if random_number % 2 == 0 {
        // println!("Result is head, user {}!!!🎉", win_or_lose);
        ctx.accounts.coin_flip_state.game_result = 1;
    } else {
        // println!("Result is tail, user {}...😏", win_or_lose);
        ctx.accounts.coin_flip_state.game_result = 2;
    };











    // if random_number % 2 == side - 1 {  // 0 for heads, 1 for tails
    //     if side == 1 {
    //         println!("The result is head, player WIN!!!")
    //     } else {
    //         println!("The result is tail, player WIN!!!")
    //     }
    // } else {
    //     if side == 1 {
    //         println!("The result is head, player LOSE!!!")
    //     } else {
    //         println!("The result is tail, player LOSE!!!")
    //     }
    // };

    // Return the result.

    // let random_bool: bool = rand::random();
    // println!("Random boolean value: {}", random_bool);

    // if result == side {
    //     println!("player choose {}, WIN!!!", side);
    // } else {
    //     println!("player choose {}, LOSE!!!", side);
    // }
    Ok(())
}

