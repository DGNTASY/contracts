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
        space = 8 + 32 + 1 + 8,
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

pub fn handler_bet(ctx: Context<Bet>, bumps: &BetBumps) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;

    let bet_amount = escrow_account.bet_amount;

    let user_token_balance = ctx.accounts.user_token_account.amount;
    require!(
        user_token_balance >= bet_amount,
        ErrorCode::InsufficientFunds
    );

    transfer_to_escrow(ctx.accounts, bet_amount)?;

    let user_account = &mut ctx.accounts.user_account;

    user_account.owner = ctx.accounts.user.key();
    user_account.is_eligible = false;
    user_account.bump = bumps.user_account;

    Ok(())
}

fn transfer_to_escrow<'info>(accounts: &mut Bet<'info>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: accounts.user_token_account.to_account_info(),
        to: accounts.escrow_token_account.to_account_info(),
        authority: accounts.user.to_account_info(),
    };
    let cpi_program = accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    let escrow_account = &mut accounts.escrow_account;
    escrow_account.usdc_balance += amount as u128;

    Ok(())
}
