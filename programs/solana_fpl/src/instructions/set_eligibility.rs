use anchor_lang::prelude::*;
use crate::state::{EscrowAccount, UserAccount};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SetEligibility<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"escrow"],
        bump = escrow_account.bump,
        has_one = authority
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        mut, 
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub user: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SetEligibility<'info> {
    pub fn handler_set_eligibility(&mut self, payout_amount: u64) -> Result<()> {
        require!(
            self.authority.key() == self.escrow_account.authority,
            ErrorCode::Unauthorized
        );

        require!(payout_amount > 0, ErrorCode::InvalidAmount);

        self.user_account.is_eligible = true;
        self.user_account.payout_amount = payout_amount;

        Ok(())
    }
}
