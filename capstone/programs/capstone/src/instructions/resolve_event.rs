use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode, Event, Outcome
};



#[derive(Accounts)]
pub struct ResolveEvent<'info>{
    #[account(mut)]
    pub event: Account<'info, Event>
}

