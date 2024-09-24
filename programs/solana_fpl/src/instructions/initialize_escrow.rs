use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::EscrowAccount;

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        seeds = [b"escrow"],
        bump,
        payer = owner, 
        space = 8 + 32 + 32 + 8 + 8 + 16 + 1,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = usdc_mint,
        associated_token::authority = escrow_account,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub usdc_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeEscrow<'info> {
    pub fn handler_initialize_escrow(
        &mut self,
        total_pot_for_winners: u64,
        bet_amount: u64,
        bumps: &InitializeEscrowBumps
    ) -> Result<()> {
        self.escrow_account.set_inner(EscrowAccount {
            authority: self.owner.key(),
            usdc_mint: self.usdc_mint.key(),
            total_pot_for_winners,
            bet_amount,
            usdc_balance: 0,
            bump: bumps.escrow_account,
        });

        Ok(())
    }
}
