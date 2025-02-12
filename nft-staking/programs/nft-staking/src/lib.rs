use anchor_lang::prelude::*;

declare_id!("C6WghYhY8D4FGcVEU79DqzrbvsPvF7XEJVz8nB9UuYMV");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
