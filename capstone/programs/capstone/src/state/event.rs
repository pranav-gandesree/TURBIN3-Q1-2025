use anchor_lang::prelude::*;

#[account]
pub struct Event{
    pub event_id: u64,

    pub title: String, //max 100chars

    pub creator: Pubkey,
    pub creation_date: i64,

    pub resolved: bool,

    pub outcomes: [Pubkey; 2], // store Yes & No outcome PDAs, NOT entire structs
    pub winning_outcome: Option<u8>, //Some(0) -> No, Some(1) -> Yes when resolved

    // pub tags: Vec<String>, // tags (max 10 tags of 10 chars each) implement later

    pub win_pool: Pubkey, //when users bet the amount will go to this pubkey owned by event pda

    pub seed: u64,
    pub event_bump: u8,

    pub aggregator: Pubkey,

}


impl Space for Event {
    const INIT_SPACE: usize = 8 + (4 + 100) + 8 + 32 + 1 + (2 * 32) + 2 + 32 + 8 + 1 + 32;
}
