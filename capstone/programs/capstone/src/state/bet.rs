use anchor_lang::prelude::*;

#[account]
pub struct Bet{
    pub better: Pubkey,

    pub event: Pubkey,

    pub outcome_index: u8,
    pub claimed: bool,

    pub creation_date: i64,

    pub bet_amount: u64,

    pub seed: u64,
    pub bump: u8

}


impl Space for Bet {
    const INIT_SPACE: usize = 32 + 32 + 1 + 1 + 8 + 8 + 8 + 1 ;
}

