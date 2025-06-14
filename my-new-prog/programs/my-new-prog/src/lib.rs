use anchor_lang::prelude::*;

use anchor_lang::solana_program::system_instruction;
use switchboard_on_demand::accounts::RandomnessAccountData; // For system program CPI
// use spl_pod::PodBool;

// pub mod accounts;      // This tells Rust to look for src/accounts/mod.rs
// pub mod instructions;  // This tells Rust to look for src/instructions/mod.rs
pub mod errors; // This tells Rust to look for src/errors.rs
pub mod events; // This tells Rust to look for src/events.rs
                // pub mod accounts;

// use crate::accounts::*;
use crate::errors::ErrorCode;
use crate::events::{
    BetCancelled, BetPlaced, DieRollTriggered, TreasuryWithdrawn, WinningsClaimed,
};

// pub mod bet_state_accounts;

// --- HOW TO LOAD ACCOUNTS ---
// You can use `use crate::accounts::*` to bring all public items
// (which include your account structs) from the `accounts` module into scope.
// use crate::accounts::*; // This brings GlobalState, TreasuryAccount, RollState, BetState into scope

// You also need to explicitly use specific items from other modules
// use crate::errors::ErrorCode;
// use crate::events::{BetPlaced, BetCancelled, WinningsClaimed, DieRollTriggered, TreasuryWithdrawn, DieRollRevealed};

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
        _max_bet_config: u64, // Renamed to `_max_bet_config` to indicate it's unused if not stored
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.authority = ctx.accounts.authority.key();
        global_state.treasury_pda = ctx.accounts.treasury_pda_account.key();
        // FIX: Access bump directly as a field, not with .get()
        global_state.treasury_bump = ctx.bumps.treasury_pda_account;
        // FIX: Access bump directly as a field, not with .get()
        global_state.bump = ctx.bumps.global_state;
        // `_max_bet_config` is not stored in GlobalState anymore, if it was intended to be
        Ok(())
    }

    // pub fn place_bet(ctx: Context<PlaceBet>, guess: u8, amount: u64) -> Result<()> {
    
    //     require!(guess >= 1 && guess <= 6, ErrorCode::InvalidGuess);

    //     // Enforce bet limits from constants
    //     require!(amount >= MIN_BET_LAMPORTS, ErrorCode::BetTooSmall);
    //     require!(amount <= MAX_BET_LAMPORTS, ErrorCode::BetTooLarge);

    //     let bet_state = &mut ctx.accounts.bet_state;
    //     // Ensure only one bet per (player, roll_state) combination is being initialized
    //     require!(bet_state.amount == 0, ErrorCode::AlreadyBet);

    //     // Check if there are any unclaimed winnings from a previous, settled roll.
    //     if let Some(previous_bet_state_account) = &ctx.accounts.previous_bet_state {
    //         let previous_bet = previous_bet_state_account.load()?;

    //         let previous_roll_state_account = match &ctx.accounts.previous_roll_state {
    //             Some(acc) => acc,
    //             None => return Err(ErrorCode::InvalidPreviousRollAccount.into()),
    //         };

    //         require!(
    //             previous_roll_state_account.key() == previous_bet.roll,
    //             ErrorCode::InvalidPreviousRollAccount
    //         );

    //         // Check if the previous roll has revealed its result AND the previous bet is NOT claimed
    //         if previous_roll_state_account.revealed && !previous_bet.claimed {
    //             return Err(ErrorCode::PreviousBetUnclaimed.into());
    //         }
    //     }

    //     bet_state.player = ctx.accounts.player.key();
    //     bet_state.roll = ctx.accounts.roll_state.key();
    //     bet_state.guess = guess;
    //     bet_state.amount = amount;
    //     bet_state.claimed = false;
    //     bet_state.bump = *ctx.bumps.get("bet_state").unwrap();

    //     // Transfer funds from player to treasury PDA
    //     anchor_lang::solana_program::program::invoke(
    //         &system_instruction::transfer(
    //             ctx.accounts.player.key,
    //             ctx.accounts.treasury_pda_account.key,
    //             amount,
    //         ),
    //         &[
    //             ctx.accounts.player.to_account_info(),
    //             ctx.accounts.treasury_pda_account.to_account_info(),
    //             ctx.accounts.system_program.to_account_info(),
    //         ],
    //     )?;

    //     // Update the total_bets_amount on the RollState
    //     let roll_state = &mut ctx.accounts.roll_state; // Get mutable reference to roll_state
    //     roll_state.total_bets_amount = roll_state
    //         .total_bets_amount
    //         .checked_add(amount)
    //         .ok_or(ErrorCode::MathOverflow)?;

    //     emit!(BetPlaced {
    //         user: ctx.accounts.player.key(), // User who placed the bet
    //         amount,
    //     });

    //     Ok(())
    // }

    
}


#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(
        init,
        payer = authority,
        // Space for GlobalState: 8 (discriminator) + 32 (Pubkey) + 32 (Pubkey) + 1 (u8) + 1 (u8) = 74 bytes
        space = 8 + 32 + 32 + 1 + 1, // Ensure this matches actual struct size
        seeds = [b"global-state"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8, // Just for Anchor's discriminator on TreasuryAccount
        seeds = [b"treasury", authority.key().as_ref()],
        bump
    )]
    /// CHECK: This is the treasury PDA, its existence and ownership is checked by Anchor.
    pub treasury_pda_account: Account<'info, TreasuryAccount>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct PlaceBet<'info> {
//     #[account(mut)]
//     pub player: Signer<'info>,

//     #[account(mut, seeds = [b"global-state"], bump = global_state.bump)]
//     pub global_state: Account<'info, GlobalState>,

//     #[account(mut, seeds = [b"roll", roll_state.randomness_account.as_ref()], bump = roll_state.bump)]
//     pub roll_state: Account<'info, RollState>,

//     #[account(
//         init,
//         payer = player,
//         // New space calculation: sum of fields, NO 8-byte discriminator
//         space = 32 + 32 + 1 + 8 + 1 + 1 + 1, // Player(32) + RollStateKey(32) + Guess(1) + Amount(8) + HasWon(1) + Redeemed(1) + Bump(1) = 76 bytes
//         seeds = [b"bet", roll_state.key().as_ref(), player.key().as_ref()],
//         bump
//     )]
//     pub bet_state: Account<'info, BetState>, // This Account is still what you interact with in the instruction
    

//     // The treasury PDA account for receiving funds
//     #[account(
//         mut,
//         seeds = [b"treasury", global_state.authority.key().as_ref()],
//         bump = global_state.treasury_bump
//     )]
//     /// CHECK: only SOL transferred
//     pub treasury_pda_account: AccountInfo<'info>,

//     pub system_program: Program<'info, System>,

//     // Optional accounts to check for previous unclaimed winnings.
//     // The client will provide these if one exists and needs checking.
//     // Using `AccountLoader` means the data is not deserialized by default.
//     // You must call `.load()` on it in your instruction logic.
//     #[account(has_one = player @ ErrorCode::PreviousBetDoesNotBelongToPlayer)]
//     pub previous_bet_state: Option<AccountLoader<'info, BetState>>,

//     // This account is required if `previous_bet_state` is provided, to verify its roll status.
//     // It must be the RollState associated with the `previous_bet_state`.
//     pub previous_roll_state: Option<Account<'info, RollState>>,
// }

// Account definitions required for initialize_contract
#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub treasury_pda: Pubkey,
    pub treasury_bump: u8, // This field IS needed to store the bump seed for future PDA signing
    pub bump: u8,          // GlobalState's own bump
}

#[account]
pub struct RollState {
    pub randomness_account: Pubkey,
    pub revealed: bool,
    pub result: Option<u8>,
    pub total_bets_amount: u64, // Total amount bet on this specific roll
    pub bump: u8,
}

// #[account(zero_copy)]
// #[repr(packed)]
// pub struct BetState {
//     pub player: Pubkey,        // 32 bytes
//     pub roll_state_key: Pubkey, // 32 bytes
//     pub guess: u8,             // 1 byte
//     pub amount: u64,           // 8 bytes
//     pub has_won: PodBool,      // 1 byte (from spl-pod)
//     pub redeemed: PodBool,     // 1 byte (from spl-pod)
//     pub bump: u8,              // 1 byte
//     // Total: 32 + 32 + 1 + 8 + 1 + 1 + 1 = 76 bytes
//     // Note: #[zero_copy] accounts do NOT have the 8-byte Anchor discriminator.
//     // So the space calculation in `PlaceBet` will need to be adjusted.
// }


#[account]
pub struct TreasuryAccount {
    // This struct holds no custom data, just SOL. Anchor adds an 8-byte discriminator.
}