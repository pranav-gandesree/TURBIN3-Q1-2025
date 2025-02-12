use anchor_lang::prelude::*;

declare_id!("2nsvFLa7JogwJqpsUZFSpDuN5CrwA6YSTWmPnbizaCHM");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
