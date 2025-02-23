
pub mod state;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use state::*;
pub use instructions::*;

declare_id!("DwbLkzCHT1AkCJaiQa93vYcQrFA9yVuc82uZ9m5EK3Ev");

#[program]
pub mod capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
