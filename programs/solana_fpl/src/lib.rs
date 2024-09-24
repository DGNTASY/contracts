use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5WuBLvXbAuBAoyysQFWyifpyevPK8tvcaTzXSiuJqrF2");

#[program]
pub mod solana_fpl {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        total_pot_for_winners: u64,
        bet_amount: u64,
    ) -> Result<()> {
        ctx.accounts.handler_initialize_escrow(
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

    pub fn claim_winner(ctx: Context<ClaimWinner>) -> Result<()> {
        ctx.accounts.handler_claim_winner()
    }
}
