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

    pub fn initialize_user(ctx: Context<InitializeUser>, seed: u64)-> Result<()>{

        ctx.accounts.initialize_user(seed, &ctx.bumps)?;

        Ok(())
    }

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_id: u64,
        title: String,
        seed: u64,
    )-> Result<()> {
       ctx.accounts.create_event(event_id, title, seed, &ctx.bumps)?;

        Ok(())
    }

    pub fn initialize_outcomes(
        ctx: Context<InitializeOutcomes>,
        outcome_yes_id: u64,
        outcome_no_id: u64,
        outcome_yes_seed: u64,
        outcome_no_seed: u64
    ) -> Result<()>{
        ctx.accounts.initialize_outcomes(outcome_yes_id, outcome_no_id, outcome_yes_seed, outcome_no_seed, &ctx.bumps)?;
        Ok(())
    }
}
