use anchor_lang::prelude::*;

use crate::User;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeUser<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", authority.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {

    pub fn initialize_user(&mut self, seed: u64, bumps: &InitializeUserBumps)-> Result<()>{

        // self.user.user = self.authority.key();
        // self.user.total_bets = 0;
        // self.user.seed = seed as u64;
        // self.user.bump = bumps.user;

        //use set_inner function when you are completely initializing a new accounts data 
        //and not modifying any paritcular field
        self.user.set_inner(User { 
            user: self.authority.key(),
            total_bets: 0,
            seed,
            bump: bumps.user,
        });
        

        Ok(())
    }
}