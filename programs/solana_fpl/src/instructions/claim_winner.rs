use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::error::ErrorCode;
use crate::state::{EscrowAccount, UserAccount};

#[derive(Accounts)]
pub struct ClaimWinner<'info> {
    #[account(
        mut,
        seeds = [b"escrow"],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimWinner<'info> {
    pub fn handler_claim_winner(&mut self) -> Result<()> {
        require!(self.user_account.is_eligible, ErrorCode::NotEligible);

        let payout_amount = self.user_account.payout_amount;
        require!(payout_amount > 0, ErrorCode::InvalidAmount);

        self.transfer_to_user(payout_amount)?;

        self.user_account.is_eligible = false;
        self.user_account.payout_amount = 0;

        Ok(())
    }

    fn transfer_to_user(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.escrow_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.escrow_account.to_account_info(),
        };
        let seeds = &[b"escrow".as_ref(), &[self.escrow_account.bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        token::transfer(cpi_ctx, amount)?;

        self.escrow_account.usdc_balance = self
            .escrow_account
            .usdc_balance
            .checked_sub(amount as u128)
            .ok_or(ErrorCode::Underflow)?;

        Ok(())
    }
}