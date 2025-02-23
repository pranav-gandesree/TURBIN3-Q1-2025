use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bets: Vec<Pubkey>,
    pub seed: u64,
    pub bump: u8,
}

impl Space for User{
    const INIT_SPACE: usize = 24 + 8 + 1;
}