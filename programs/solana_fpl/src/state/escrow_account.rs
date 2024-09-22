use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub total_pot_for_winners: u64,
    pub bet_amount: u64,
    pub usdc_balance: u128,
    pub bump: u8,
}