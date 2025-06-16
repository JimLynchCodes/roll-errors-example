// programs/sb_randomness/src/accounts/treasury_account.rs

use anchor_lang::prelude::*;

#[account]
pub struct TreasuryAccount {
    // This struct doesn't need any custom fields if its sole purpose is to hold SOL.
    // Anchor automatically adds an 8-byte discriminator to all `#[account]` structs,
    // which serves as a unique identifier for accounts owned by this program.
}

impl TreasuryAccount {
    // The space for this account will just be Anchor's 8-byte discriminator.
    // No custom fields means its data length is 0.
    pub const LEN: usize = 0; // The data portion of this account is 0 bytes.
}