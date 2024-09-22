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
        bump,
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

pub fn handler_set_eligibility(
    ctx: Context<SetEligibility>,
    payout_amount: u64
) -> Result<()> {

    let user_account = &mut ctx.accounts.user_account;
    let escrow_account = &ctx.accounts.escrow_account;

    require!(
        ctx.accounts.authority.key() == escrow_account.authority,
        ErrorCode::Unauthorized
    );

    user_account.is_eligible = true;
    user_account.payout_amount = payout_amount;
    Ok(())
}
