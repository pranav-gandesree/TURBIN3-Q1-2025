pub mod constants;
pub mod state;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;
pub use instructions::*;

declare_id!("DwbLkzCHT1AkCJaiQa93vYcQrFA9yVuc82uZ9m5EK3Ev");

#[program]
pub mod capstone {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_id: u64,
        title: String,
        seed: u64,
    )-> Result<()> {
       ctx.accounts.create_event(event_id, title, seed, &ctx.bumps)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
