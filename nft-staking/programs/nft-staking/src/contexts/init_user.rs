use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::User;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, seeds = [b"user".as_ref()], bump, space = User::INIT_SPACE)]
    pub user_account: Account<'info, User>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps) -> Result<()> {
        self.user_account.set_inner(User {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }
}