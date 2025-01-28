use anchor_lang::prelude::*;

use crate::instructions::*;
declare_id!("8o4NHsfNw24k9Ln9wKm24PsbRut5PYy3g6QmzH7BmcRk");


pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;


#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
