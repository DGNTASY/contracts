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
}