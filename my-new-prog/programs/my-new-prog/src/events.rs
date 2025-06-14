// programs/sb_randomness/src/events.rs

use anchor_lang::prelude::*; // Brings in the #[event] macro and Pubkey type

#[event]
pub struct BetPlaced {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct BetCancelled {
    pub user: Pubkey,
}

#[event]
pub struct WinningsClaimed {
    pub user: Pubkey,
    pub amount: u64, // Amount net received by player (after commission)
}

#[event]
pub struct DieRollTriggered {
    pub user: Pubkey, // The user who initiated the roll (e.g., the game operator)
}

#[event]
pub struct TreasuryWithdrawn {
    pub user: Pubkey, // The authority who performed the withdrawal
    pub amount: u64,
}

#[event]
pub struct DieRollRevealed {
    pub result: u8,
    pub randomness: [u8; 32], // Raw 32-byte randomness from Switchboard
}