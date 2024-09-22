use anchor_lang::prelude::*;

use crate::state::EscrowAccount;

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        seeds = [b"escrow"],
        bump,
        payer = owner, 
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 16,
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_initialize_escrow(
    ctx: Context<InitializeEscrow>,
    usdc_mint: Pubkey,
    total_pot_for_winners: u64,
    bet_amount: u64,
    bumps: &InitializeEscrowBumps
) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;

    escrow_account.authority = ctx.accounts.owner.key();
    escrow_account.usdc_mint = usdc_mint;

    escrow_account.total_pot_for_winners = total_pot_for_winners;

    escrow_account.bet_amount = bet_amount;
    escrow_account.usdc_balance = 0;
    escrow_account.bump = bumps.escrow_account;

    Ok(())
}
