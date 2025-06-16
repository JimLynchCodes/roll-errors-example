// programs/sb_randomness/src/errors.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // --- Bet-related Errors ---
    #[msg("Invalid guess. Must be between 1 and 6.")]
    InvalidGuess,
    #[msg("Bet amount exceeds maximum allowed.")]
    BetTooLarge,
    #[msg("Bet amount is below minimum allowed.")]
    BetTooSmall,
    #[msg("Bet already placed for this roll.")]
    AlreadyBet,
    #[msg("Cannot cancel bet, roll has already been settled.")]
    RollAlreadySettled,
    #[msg("You have unclaimed winnings from a previous bet that must be settled first.")]
    PreviousBetUnclaimed,
    #[msg("The provided previous bet state does not belong to the player.")]
    PreviousBetDoesNotBelongToPlayer,
    #[msg("Invalid previous roll account provided.")]
    InvalidPreviousRollAccount,
    #[msg("Already claimed winnings.")]
    AlreadyClaimed,

    // --- Roll-related Errors ---
    #[msg("Randomness not yet revealed.")]
    RollNotSettled,
    #[msg("Randomness already revealed.")]
    AlreadySettled,
    #[msg("Randomness not resolved. Switchboard Oracle has not provided a result yet.")]
    RandomnessNotResolved,
    #[msg("Insufficient treasury funds to trigger a new roll. Pot needs more SOL.")]
    InsufficientTreasuryForRoll,

    // --- Treasury/Funds Errors ---
    #[msg("Insufficient treasury funds to cover payout.")]
    InsufficientTreasury,
    #[msg("Unauthorized withdrawal from treasury.")]
    UnauthorizedWithdraw,
    #[msg("Insufficient treasury funds for withdrawal. Cannot withdraw below minimum pot.")]
    InsufficientTreasuryForWithdrawal,

    // --- Arithmetic Errors ---
    #[msg("Arithmetic overflow occurred.")]
    MathOverflow,
    #[msg("Arithmetic underflow occurred.")]
    MathUnderflow,
}