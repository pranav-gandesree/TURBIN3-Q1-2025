use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub user: Pubkey,       // user's wallet address
    pub total_bets: u64,    // number of bets placed
    pub seed: u64,          // PDA seed
    pub bump: u8            // PDA bump
}

impl Space for User {
    const INIT_SPACE: usize = 32 + 8 + 8 + 1;
}

#[account]
pub struct UserBet {
    pub user: Pubkey,       // user who placed the bet
    pub bet: Pubkey,        // the bet account
    pub seed: u64,          // PDA seed
    pub bump: u8            // PDA bump
}

impl Space for UserBet {
    const INIT_SPACE: usize = 32 + 32 + 8 + 1;
}
