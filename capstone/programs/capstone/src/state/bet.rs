use anchor_lang::prelude::*;

#[account]
pub struct Bet{
    pub better: Pubkey, // user who placed the bet

    pub event: Pubkey,  //event this bet belongs to
    pub outcome: Pubkey, // the outcome this bet has placed on
    pub outcome_index: u8, // 0 = No, 1 = Yes

    pub claimed: bool,  //if the winner claimed his bet

    pub creation_date: i64,

    pub bet_amount: u64,    //amount user has bet

    pub seed: u64,
    pub bump: u8

}


impl Space for Bet {
    const INIT_SPACE: usize = 32 + 32 + 32 + 1 + 1 + 8 + 8 + 8 + 1 ;
}

