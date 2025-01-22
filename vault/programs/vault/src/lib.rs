use anchor_lang::prelude::*;

declare_id!("F7v8dZYRXZhiUGXu8RySMSRPe49kZr1iXCPdSv2XQxpy");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
}




#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        space = VaultState :: INIT_SPACE,
        payer = user,
        seeds = [b"state",user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        
        Ok(())
    }
}




#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}

impl Space for VaultState{
    const INIT_SPACE: usize = 8 + 1 + 1; //8 for descriminator, 1 for vault_bump and state_bump
}