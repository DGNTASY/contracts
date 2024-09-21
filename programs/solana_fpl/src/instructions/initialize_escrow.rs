use anchor_lang::prelude::*;

use crate::state::EscrowAccount;

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 8 + 8 + 16 + 8)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_initialize_escrow(
    ctx: Context<InitializeEscrow>,
    usdc_mint: Pubkey,
    payout_first: u64,
    payout_second: u64,
    payout_third: u64,
    bet_amount: u64,
) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;

    escrow_account.authority = ctx.accounts.owner.key();
    escrow_account.usdc_mint = usdc_mint;

    escrow_account.payout_first = payout_first;
    escrow_account.payout_second = payout_second;
    escrow_account.payout_third = payout_third;

    escrow_account.bet_amount = bet_amount;
    escrow_account.usdc_balance = 0;

    Ok(())
}
