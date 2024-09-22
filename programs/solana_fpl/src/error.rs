use anchor_lang::prelude::*;

// Define custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount provided.")]
    InvalidAmount,
    #[msg("Insufficient funds in the user's token account.")]
    InsufficientFunds,
    #[msg("Unauthorized to perform this action")]
    Unauthorized,
    #[msg("Overflow while adding bet amount to escrow balance")]
    Overflow,
    #[msg("Underflow while passing payout from escrow to user")]
    Underflow,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("User is not eligible to claim")]
    NotEligible,
}