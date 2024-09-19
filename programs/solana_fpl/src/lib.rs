use anchor_lang::prelude::*;

declare_id!("8j5Fm6UKtk7rV4xZGXEQZcHZnSKqxx6aEeSkHEac7K2W");

#[program]
pub mod solana_fpl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
