use anchor_lang::prelude::*;

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
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeEscrow<'info> {
    pub fn handler_initialize_escrow(
        &mut self,
        usdc_mint: Pubkey,
        total_pot_for_winners: u64,
        bet_amount: u64,
        bumps: &InitializeEscrowBumps
    ) -> Result<()> {
        self.escrow_account.set_inner(EscrowAccount {
            authority: self.owner.key(),
            usdc_mint,
            total_pot_for_winners,
            bet_amount,
            usdc_balance: 0,
            bump: bumps.escrow_account,
        });

        Ok(())
    }
}
