// programs/sb_randomness/src/accounts/global_state.rs

use anchor_lang::prelude::*;

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
    // No explicit LEN constant needed for regular `#[account]` structs as Anchor handles space calculation directly
    // when using `space = 8 + <size_of_fields>`.
}