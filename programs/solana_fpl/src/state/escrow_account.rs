use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub payout_first: u64,
    pub payout_second: u64,
    pub payout_third: u64,
    pub bet_amount: u64,
    pub usdc_balance: u128,
}