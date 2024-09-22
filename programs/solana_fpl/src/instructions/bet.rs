use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::error::ErrorCode;
use crate::state::{EscrowAccount, UserAccount};

#[derive(Accounts)]
pub struct Bet<'info> {
    #[account(
        mut,
        seeds = [b"escrow"],
        bump = escrow_account.bump,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(
        init,
        seeds = [b"user", user.key().as_ref()],
        bump,
        payer = user, 
        space = 8 + 32 + 1 + 8 + 1,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Bet<'info> {
    pub fn handler_bet(&mut self, bumps: &BetBumps) -> Result<()> {
        let bet_amount = self.escrow_account.bet_amount;

        let user_token_balance = self.user_token_account.amount;
        require!(
            user_token_balance >= bet_amount,
            ErrorCode::InsufficientFunds
        );

        self.transfer_to_escrow(bet_amount)?;

        self.user_account.owner = self.user.key();
        self.user_account.is_eligible = false;
        self.user_account.bump = bumps.user_account;

        Ok(())
    }

    fn transfer_to_escrow(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.escrow_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        self.escrow_account.usdc_balance += amount as u128;

        Ok(())
    }
}