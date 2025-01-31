use anchor_lang::prelude::*;

declare_id!("24RJK9p2YJ3rrWjqufgNjKyPxi1VZUDGnb7K2xGDChe9");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
