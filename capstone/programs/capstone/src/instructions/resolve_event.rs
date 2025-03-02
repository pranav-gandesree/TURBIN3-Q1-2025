use anchor_lang::prelude::*;

use crate::{
Event, Outcome

};



#[derive(Accounts)]
pub struct ResolveEvent<'info>{

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
          b"EVENT",
          authority.key().as_ref(),
          event.event_id.to_le_bytes().as_ref(),
          event.seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [
            b"OUTCOME",
            event.key().as_ref(),
            &outcome_yes.outcome_id.to_le_bytes().as_ref(),
            &outcome_yes.seed.to_le_bytes()
            ],
        bump,
    )]
    pub outcome_yes: Account<'info, Outcome>,

    #[account(
        mut,
        seeds = [
            b"OUTCOME",
            event.key().as_ref(),
            &outcome_no.outcome_id.to_le_bytes().as_ref(),
            &outcome_no.seed.to_le_bytes()
            ],
        bump,
    )]
    pub outcome_no: Account<'info, Outcome>,

    #[account(mut)]
    pub switchboard_feed: AccountInfo<'info>,
    

    pub system_program: Program<'info, System>,


}


impl<'info> ResolveEvent<'info> {
    pub fn resolve_event(&mut self) -> Result<()> {
    //     // Fetch the event result from Switchboard
    //     // let switchboard_result = Self::fetch_switchboard_result(&self.switchboard_feed)?;

    //     // Determine event outcome (Yes/No)
    //     let resolved_outcome = if switchboard_result > 0.5 { 1 } else { 0 };

    //     // Update the event result
    //     self.event.winning_outcome = Some(resolved_outcome);

    //     // Update the corresponding outcome
    //     if resolved_outcome == 1 {
    //         self.outcome_yes.resolved = true;
    //     } else {
    //         self.outcome_no.resolved = true;
    //     }

    //     msg!(
    //         "Event {} resolved: {}",
    //         self.event.event_id,
    //         if resolved_outcome == 1 { "Yes" } else { "No" }
    //     );

        Ok(())
    }

    // fn fetch_switchboard_result(&self,  feed: &AccountInfo) -> Result<f64> {
    //     // Deserialize the Switchboard account
        
    //     msg!("Switchboard fetched value: {}", value);
    //     Ok(value)
    // }
}

