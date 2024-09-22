use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub user: Pubkey,
    pub is_eligible: bool,
    pub payout_amount: u64,
}