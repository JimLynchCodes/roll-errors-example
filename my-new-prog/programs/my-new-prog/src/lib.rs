use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use switchboard_on_demand::accounts::RandomnessAccountData;

pub mod errors;
pub mod events;

use crate::errors::ErrorCode;
use crate::events::{
    BetCancelled, BetPlaced, DieRollTriggered, TreasuryWithdrawn, WinningsClaimed,
};

declare_id!("FRb5eZnHH434Z5tQzoifEVL5MC8XCs4t3jXkkraszuZg");

// Define constants for bet limits in lamports
const MIN_BET_LAMPORTS: u64 = 1_000_000; // 0.001 SOL
const MAX_BET_LAMPORTS: u64 = 100_000_000; // 0.1 SOL
const MIN_POT_FOR_ROLL_LAMPORTS: u64 = 100_000_000; // 0.1 SOL

#[program]
pub mod my_new_prog {
    use super::*;

    pub fn initialize_contract(
        ctx: Context<InitializeContract>,
        _max_bet_config: u64,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.authority = ctx.accounts.authority.key();
        global_state.treasury_pda = ctx.accounts.treasury_pda_account.key();
        global_state.treasury_bump = ctx.bumps.treasury_pda_account;
        global_state.bump = ctx.bumps.global_state;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, guess: u8, amount: u64) -> Result<()> {
        require!(guess >= 1 && guess <= 6, ErrorCode::InvalidGuess);
        require!(amount >= MIN_BET_LAMPORTS, ErrorCode::BetTooSmall);
        require!(amount <= MAX_BET_LAMPORTS, ErrorCode::BetTooLarge);
    
        // Ensure previous bet is claimed if it exists
        if let Some(previous_bet_state_account) = &ctx.accounts.previous_bet_state {
            let previous_bet = previous_bet_state_account;
    
            let previous_roll_state_account = match &ctx.accounts.previous_roll_state {
                Some(acc) => acc,
                None => return Err(ErrorCode::InvalidPreviousRollAccount.into()),
            };
    
            require!(
                previous_roll_state_account.key() == previous_bet.roll,
                ErrorCode::InvalidPreviousRollAccount
            );
    
            if previous_roll_state_account.revealed && !previous_bet.claimed {
                return Err(ErrorCode::PreviousBetUnclaimed.into());
            }
        }
    
        // Set up new bet state
        let bet_state = &mut ctx.accounts.bet_state;
        bet_state.player = ctx.accounts.player.key();
        bet_state.roll = ctx.accounts.roll_state.key();
        bet_state.guess = guess;
        bet_state.amount = amount;
        bet_state.claimed = false;
        bet_state.bump = ctx.bumps.bet_state;
    
        // Transfer lamports to treasury
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                ctx.accounts.player.key,
                ctx.accounts.treasury_pda_account.key,
                amount,
            ),
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.treasury_pda_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    
        // Update total bets on the roll
        let roll_state = &mut ctx.accounts.roll_state;
        roll_state.total_bets_amount = roll_state
            .total_bets_amount
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;
    
        emit!(BetPlaced {
            user: ctx.accounts.player.key(),
            amount,
        });
    
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 1,
        seeds = [b"global-state"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [b"treasury", authority.key().as_ref()],
        bump
    )]
    /// CHECK: This is the treasury PDA, its existence and ownership is checked by Anchor.
    pub treasury_pda_account: Account<'info, TreasuryAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, seeds = [b"global-state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut, seeds = [b"roll", roll_state.randomness_account.as_ref()], bump = roll_state.bump)]
    pub roll_state: Account<'info, RollState>,

    #[account(
        init,
        payer = player,
        space = 8 + std::mem::size_of::<BetState>(), // 8 bytes for discriminator
        seeds = [b"bet", roll_state.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub bet_state: Account<'info, BetState>,

    #[account(
        mut,
        seeds = [b"treasury", global_state.authority.key().as_ref()],
        bump = global_state.treasury_bump
    )]
    /// CHECK: Treasury is just a SOL holding account
    pub treasury_pda_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        owner = crate::ID,
        has_one = player @ ErrorCode::PreviousBetDoesNotBelongToPlayer
    )]
    pub previous_bet_state: Option<Account<'info, BetState>>,

    pub previous_roll_state: Option<Account<'info, RollState>>,
}

// Account definitions
#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub treasury_pda: Pubkey,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[account]
pub struct RollState {
    pub randomness_account: Pubkey,
    pub revealed: bool,
    pub result: Option<u8>,
    pub total_bets_amount: u64,
    pub bump: u8,
}

#[account]
pub struct BetState {
    pub player: Pubkey,       // 32 bytes
    pub roll: Pubkey,         // 32 bytes
    pub guess: u8,            // 1 byte
    pub amount: u64,          // 8 bytes
    pub claimed: bool,        // 1 byte
    pub bump: u8,             // 1 byte (If you intend to store the bump in the account itself)
}

#[account]
pub struct TreasuryAccount {
    // This struct holds no custom data, just SOL. Anchor adds an 8-byte discriminator.
}