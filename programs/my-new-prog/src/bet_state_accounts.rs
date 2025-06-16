// programs/sb_randomness/src/accounts/bet_state.rs

use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable}; // Import Pod and Zeroable traits from bytemuck

// #[account(zero_copy)] combines #[account] and #[zero_copy] functionality.
// #[repr(C)] ensures C-like memory layout.
// #[derive(Pod, Zeroable)] explicitly marks the struct as Plain Old Data (POD)
// and ensures it can be safely initialized with zeros, satisfying bytemuck's requirements.
// #[account(zero_copy)]
#[repr(C)]
#[derive(Zeroable)]
pub struct BetState {
    // Fields ordered from largest to smallest for better natural alignment in memory
    pub player: Pubkey,   // 32 bytes - The player who placed the bet
    pub roll: Pubkey,     // 32 bytes - The Pubkey of the RollState this bet is for
    pub amount: u64,      // 8 bytes  - The amount of SOL bet
    pub guess: u8,        // 1 byte   - The player's guessed number (1-6)
    pub claimed: u8,      // CHANGED: From bool to u8 to ensure Pod compatibility (1 byte)
    pub bump: u8,         // 1 byte   - The bump seed for deriving this BetState account
    _padding: [u8; 5],    // 5 bytes  - Padding to ensure the total data size is a multiple of 8 (80 bytes total data)
}

impl BetState {
    // Defines the size of the data for BetState *without* the Anchor discriminator (8 bytes)
    pub const LEN: usize = 32 + 32 + 8 + 1 + 1 + 1 + 5; // Total 80 bytes of data
}