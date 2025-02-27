use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode, Event, Outcome
    // USDC_MINT_ADDRESS
};



#[derive(Accounts)]
#[instruction(outcome_ids: [u64; 2], seeds: [u64; 2])]
pub struct InitializeOutcomes<'info>{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        has_one = creator @ ErrorCode::UnauthorizedAccess
    )]
    pub event: Account<'info, Event>,

    #[account(
        init,
        payer = creator,
        space = 8 + Outcome::INIT_SPACE,
        seeds = [b"OUTCOME", event.key().as_ref(), outcome_ids[0].to_le_bytes().as_ref(),
        seeds[0].to_le_bytes().as_ref()],
        bump
    )]
    pub outcome_no: Account<'info, Outcome>,

    #[account(
        init,
        payer = creator,
        space = 8 + Outcome::INIT_SPACE,
        seeds = [b"OUTCOME", event.key().as_ref(), outcome_ids[1].to_le_bytes().as_ref(),
        seeds[1].to_le_bytes().as_ref()],
        bump
    )]
    pub outcome_yes: Account<'info, Outcome>,


    pub system_program: Program<'info, System>
}
    


impl<'info> InitializeOutcomes<'info>{

    pub fn initialize_outcomes(
        &mut self,
        outcome_ids: [u64; 2],
        seeds: [u64; 2],
        outcome_yes_bump: u8,  
        outcome_no_bump: u8     
    )-> Result<()>{
        let clock = Clock::get()?;

          // Initialize No Outcome
          self.outcome_no.outcome_id = outcome_ids[0]; 
          self.outcome_no.outcome_index = 0;
          self.outcome_no.resolved = false;
          self.outcome_no.event_id = self.event.key();
          self.outcome_no.creation_date = clock.unix_timestamp;
          self.outcome_no.shares = 0;
          self.outcome_no.total_liquidity = 0;
          self.outcome_no.seed = seeds[0];
          self.outcome_no.bump = outcome_no_bump;

            // Initialize Yes Outcome
            self.outcome_yes.outcome_id = outcome_ids[1]; 
            self.outcome_yes.outcome_index =1;
            self.outcome_yes.resolved = false;
            self.outcome_yes.event_id = self.event.key();
            self.outcome_yes.creation_date = clock.unix_timestamp;
            self.outcome_yes.shares = 0;
            self.outcome_yes.total_liquidity = 0;
            self.outcome_yes.seed = seeds[1];
            self.outcome_yes.bump = outcome_yes_bump;
    
          

        Ok(())
    }
}