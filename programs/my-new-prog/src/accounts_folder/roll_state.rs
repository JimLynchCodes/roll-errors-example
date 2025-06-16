// programs/sb_randomness/src/accounts/roll_state.rs

use anchor_lang::prelude::*;

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
    // 8 (discriminator) + 32 (Pubkey) + 1 (bool) + 1 (Option<u8> tag) + 1 (Option<u8> value) + 8 (u64) + 1 (u8) = 52 bytes
    // Note: Option<u8> takes 2 bytes (1 byte for tag, 1 byte for value)
    pub const LEN: usize = 32 + 1 + 2 + 8 + 1; // Sum of fields without Anchor's 8-byte discriminator
}