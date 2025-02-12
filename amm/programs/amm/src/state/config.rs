use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64,                 // seed to be able to create different pools/configs

    pub authority: Option<Pubkey>, // if we want to authority to lock the config account

    pub mint_x: Pubkey,
    pub mint_y: Pubkey,

    pub fee: u16,                  // swap fee in basis points
    pub locked: bool,              // if the pool is locked
    
    pub config_bump: u8,           // bump seed for the config account
    pub lp_bump: u8                // bump seed for the lp token
}
