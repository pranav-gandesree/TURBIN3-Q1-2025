use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TokenInterface} 
    // token::{Mint, Token, TokenAccount}
};

use crate::{
    error::ErrorCode,
    Event,
    // USDC_MINT_ADDRESS
};



#[derive(Accounts)]
#[instruction(event_id: u64, title: String, seed: u64)]
pub struct CreateEvent<'info>{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Event::INIT_SPACE,
        seeds = [
          b"EVENT",
          creator.key().as_ref(),
          event_id.to_le_bytes().as_ref(),
          seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub event: Account<'info, Event>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = usdc_mint,
        associated_token::authority = event
    )]
    pub win_pool: InterfaceAccount<'info, TokenAccount>,

    // #[account(address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap())]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}


impl<'info> CreateEvent<'info>{

    pub fn create_event(
        &mut self,
        event_id: u64,
        title: String,
        seed: u64,
        bumps: &CreateEventBumps,
    )-> Result<()> {

       require!(title.len() <= 100, ErrorCode::TitleTooLong);

       self.event.set_inner( Event {
            event_id,
            title,
            creator: self.creator.key(),
            creation_date: Clock::get()?.unix_timestamp,
            resolved: false,
            outcomes: [Pubkey::default(), Pubkey::default()],
            winning_outcome: None,
            win_pool: self.win_pool.key(),
            seed,
            event_bump: bumps.event,
            
       });
    
        Ok(())
    
    }
}