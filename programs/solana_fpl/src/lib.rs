use anchor_lang::prelude::*;

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
        payout_first: u64,
        payout_second: u64,
        payout_third: u64,
        bet_amount: u128,
    ) -> Result<()> {
        instructions::initialize_escrow::handler_initialize_escrow(
            ctx,
            usdc_mint,
            payout_first,
            payout_second,
            payout_third,
            bet_amount,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}
