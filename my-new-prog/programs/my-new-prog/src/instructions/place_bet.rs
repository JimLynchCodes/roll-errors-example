// programs/sb_randomness/src/instructions/place_bet.rs

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

// Import accounts and errors from your crate
// use crate::accounts::{GlobalState, TreasuryAccount, RollState, BetState};
use crate::errors::ErrorCode;
use crate::events::BetPlaced;

// Import constants from the crate root (lib.rs)
use crate::{MIN_BET_LAMPORTS, MAX_BET_LAMPORTS};


#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(seeds = [b"global-state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut, seeds = [b"roll", roll_state.randomness_account.as_ref()], bump = roll_state.bump)]
    pub roll_state: Account<'info, RollState>,
    #[account(
        init,
        payer = player,
        space = 8 + BetState::LEN,
        seeds = [b"bet", roll_state.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub bet_state: AccountLoader<'info, BetState>,
    #[account(
        mut,
        seeds = [b"treasury", global_state.authority.key().as_ref()],
        bump = global_state.treasury_bump
    )]
    /// CHECK: This is the treasury PDA. Funds are transferred into it.
    pub treasury_pda_account: Account<'info, TreasuryAccount>,
    pub system_program: Program<'info, System>,

    #[account(has_one = player @ ErrorCode::PreviousBetDoesNotBelongToPlayer)]
    pub previous_bet_state: Option<AccountLoader<'info, BetState>>,

    pub previous_roll_state: Option<Account<'info, RollState>>,
}

// Handler function for the place_bet instruction
pub fn handler(ctx: Context<PlaceBet>, guess: u8, amount: u64) -> Result<()> {
    require!(guess >= 1 && guess <= 6, ErrorCode::InvalidGuess);
    require!(amount >= MIN_BET_LAMPORTS, ErrorCode::BetTooSmall);
    require!(amount <= MAX_BET_LAMPORTS, ErrorCode::BetTooLarge);

    let bet_state = &mut ctx.accounts.bet_state.load_mut()?;
    require!(bet_state.amount == 0, ErrorCode::AlreadyBet);

    if let Some(previous_bet_state_account) = &ctx.accounts.previous_bet_state {
        let previous_bet = previous_bet_state_account.load()?;

        let previous_roll_state_account = match &ctx.accounts.previous_roll_state {
            Some(acc) => acc,
            None => return Err(ErrorCode::InvalidPreviousRollAccount.into()),
        };

        require!(
            previous_roll_state_account.key() == previous_bet.roll,
            ErrorCode::InvalidPreviousRollAccount
        );

        if previous_roll_state_account.revealed && previous_bet.claimed == 0 { // Check claimed as u8
            return Err(ErrorCode::PreviousBetUnclaimed.into());
        }
    }

    bet_state.player = ctx.accounts.player.key();
    bet_state.roll = ctx.accounts.roll_state.key();
    bet_state.guess = guess;
    bet_state.amount = amount;
    bet_state.claimed = 0; // Initialize as 0 (false)
    bet_state.bump = ctx.bumps.bet_state;

    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            ctx.accounts.player.key,
            ctx.accounts.treasury_pda_account.key(),
            amount,
        ),
        &[
            ctx.accounts.player.to_account_info(),
            ctx.accounts.treasury_pda_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let roll_state = &mut ctx.accounts.roll_state;
    roll_state.total_bets_amount = roll_state.total_bets_amount.checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    emit!(BetPlaced {
        user: ctx.accounts.player.key(),
        amount,
    });

    Ok(())
}