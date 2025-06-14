// programs/sb_randomness/src/accounts.rs

use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable}; // Needed for BetState's zero_copy trait

#[account]
pub struct GlobalState {
    pub authority: Pubkey,      // The main authority controlling the program (e.g., game operator)
    pub treasury_pda: Pubkey,   // The PDA that holds the program's SOL treasury
    pub treasury_bump: u8,      // The bump seed for deriving the treasury_pda
    pub bump: u8,               // The bump seed for deriving this GlobalState account
}

impl GlobalState {
    // Defines the space needed for this account (discriminator + fields)
    // 8 (discriminator) + 32 (authority) + 32 (treasury_pda) + 1 (treasury_bump) + 1 (bump) = 74 bytes
    pub const LEN: usize = 32 + 32 + 1 + 1; // Size of data without discriminator
}

#[account]
pub struct TreasuryAccount {
    // This struct doesn't need any custom fields if its sole purpose is to hold SOL.
    // Anchor automatically adds an 8-byte discriminator to all `#[account]` structs.
}

impl TreasuryAccount {
    // The space for this account will just be Anchor's 8-byte discriminator.
    pub const LEN: usize = 0; // The data portion of this account is 0 bytes.
}

#[account]
pub struct RollState {
    pub randomness_account: Pubkey, // The Pubkey of the Switchboard On-Demand randomness account
    pub revealed: bool,             // True if the random result has been revealed
    pub result: Option<u8>,         // The dice roll result (1-6), None if not yet revealed
    pub total_bets_amount: u64,     // Total SOL amount bet on this specific roll
    pub bump: u8,                   // The bump seed for deriving this RollState account
}

impl RollState {
    // Defines the space needed for this account (discriminator + fields)
    // Option<u8> takes 2 bytes (1 for tag, 1 for value)
    pub const LEN: usize = 32 + 1 + 2 + 8 + 1; // Sum of fields without Anchor's 8-byte discriminator
}

#[account(zero_copy)]
#[repr(C)]
#[derive(Pod, Zeroable)] // Needed for zero_copy
pub struct BetState {
    pub player: Pubkey,   // 32 bytes - The player who placed the bet
    pub roll: Pubkey,     // 32 bytes - The Pubkey of the RollState this bet is for
    pub amount: u64,      // 8 bytes  - The amount of SOL bet
    pub guess: u8,        // 1 byte   - The player's guessed number (1-6)
    pub claimed: u8,      // 1 byte   - Claimed status (0 = false, 1 = true)
    pub bump: u8,         // 1 byte   - The bump seed for deriving this BetState account
    _padding: [u8; 5],    // 5 bytes  - Padding to make total data size 80 bytes (multiple of 8)
}

impl BetState {
    pub const LEN: usize = 32 + 32 + 8 + 1 + 1 + 1 + 5; // Total 80 bytes of data
}