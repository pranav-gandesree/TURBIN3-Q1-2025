use anchor_lang::prelude::*;

#[account]
pub struct Outcome{
    pub outcome_id: u64,

    pub outcome_index: u8, // 0 = No, 1 = Yes (saves space)
 
    pub resolved: bool,
    
    pub event_id: Pubkey,  //links outcome to its event

    pub creation_date: i64,

    pub shares: u64,// tracks no of shares in this outcome 
    pub total_liquidity: u64, // tracks total liquidity in this outcome

    pub seed: u64,
    pub bump: u8

}


    
impl Space for Outcome {
    const INIT_SPACE: usize = 8 + 1 + 1 +  32 + 8 + 8 + 8 + 8 + 1;
}
