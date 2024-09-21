use anchor_lang::prelude::*;

// Define custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount provided.")]
    InvalidAmount,
}