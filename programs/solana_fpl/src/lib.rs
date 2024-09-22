use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8j5Fm6UKtk7rV4xZGXEQZcHZnSKqxx6aEeSkHEac7K2W");

#[program]
pub mod solana_fpl {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        usdc_mint: Pubkey,
        total_pot_for_winners: u64,
        bet_amount: u64,
    ) -> Result<()> {
        ctx.accounts.handler_initialize_escrow(
            usdc_mint,
            total_pot_for_winners,
            bet_amount,
            &ctx.bumps,
        )
    }

    pub fn bet(ctx: Context<Bet>) -> Result<()> {
        ctx.accounts.handler_bet(&ctx.bumps)
    }

    pub fn set_eligibility(ctx: Context<SetEligibility>, payout_amount: u64) -> Result<()> {
        ctx.accounts.handler_set_eligibility(payout_amount)
    }
}
