use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CreateEvent<'info>{
    #[account(mut)]
    pub creater: Signer<'info>,

    
}


