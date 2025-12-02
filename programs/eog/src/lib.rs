use anchor_lang::prelude::*;

declare_id!("FdugZTVqxDJZfuxi3HNpzDmSSuAQ2YZkwAJyPmcFCmM8");

#[program]
pub mod eog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
