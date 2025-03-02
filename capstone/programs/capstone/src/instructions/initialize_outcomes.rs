use anchor_lang::prelude::*;

use crate::{
    calculate_lmsr_price, error::ErrorCode, Event, Outcome
};



#[derive(Accounts)]
#[instruction(outcome_yes_id: u64,outcome_no_id: u64, outcome_yes_seed: u64, outcome_no_seed: u64)]
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
        seeds = [
            b"OUTCOME",
            event.key().as_ref(),
            &outcome_yes_id.to_le_bytes().as_ref(),
            &outcome_yes_seed.to_le_bytes()
            ],
        bump,
    )]
    pub outcome_no: Account<'info, Outcome>,

    #[account(
        init,
        payer = creator,
        space = 8 + Outcome::INIT_SPACE,
        seeds = [
            b"OUTCOME",
            event.key().as_ref(),
            &outcome_no_id.to_le_bytes().as_ref(),
            &outcome_no_seed.to_le_bytes().as_ref()
            ],
        bump
    )]
    pub outcome_yes: Account<'info, Outcome>,


    pub system_program: Program<'info, System>
}
    


impl<'info> InitializeOutcomes<'info>{


    pub fn initialize_outcomes(
        &mut self,
        outcome_yes_id: u64,
        outcome_no_id: u64,
        outcome_yes_seed: u64,
        outcome_no_seed: u64,
        bumps: &InitializeOutcomesBumps    
    )-> Result<()>{
        let clock = Clock::get()?;


        // Initialize No Outcome
        self.outcome_no.set_inner(Outcome {
            outcome_id : outcome_no_id,
            outcome_index : 0,
            resolved: false,
            event_id: self.event.key(),
            creation_date: clock.unix_timestamp,
            shares: 0,
            total_liquidity: 0,
            seed: outcome_no_seed,
            bump: bumps.outcome_no
        });

            // Initialize Yes Outcome
            self.outcome_yes.set_inner(Outcome {
                outcome_id: outcome_yes_id,
                outcome_index: 1,
                resolved: false,
                event_id: self.event.key(),
                creation_date: clock.unix_timestamp,
                shares: 0,
                total_liquidity: 0,
                seed: outcome_yes_seed,
                bump: bumps.outcome_yes,
            });
    
          // Store outcome PDAs in event
       self.event.outcomes = [self.outcome_no.key(), self.outcome_yes.key()];

       // calculate here initial prise of the tokens using lmsr
        let (price_yes, price_no) = calculate_lmsr_price(0, 0);
        msg!("Initial Price: Yes = {:.4}, No = {:.4}", price_yes, price_no);
        
        Ok(())
    }
}
