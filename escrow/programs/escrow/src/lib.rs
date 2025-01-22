use anchor_lang::prelude::*;

declare_id!("8o4NHsfNw24k9Ln9wKm24PsbRut5PYy3g6QmzH7BmcRk");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
