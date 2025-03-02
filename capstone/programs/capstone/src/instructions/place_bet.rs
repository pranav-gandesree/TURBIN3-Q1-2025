use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    calculate_lmsr_price, error::ErrorCode, Bet, Event, Outcome, User, UserBet
};



#[derive(Accounts)]
#[instruction(bet_amount: u64, outcome_index: u8, seed: u64)]
pub struct PlaceBet<'info>{
    #[account(mut)]
    pub better: Signer<'info>,

    #[account(
        mut,
        constraint = !event.resolved @ ErrorCode::EventAlreadyResolved
    )]
    pub event: Account<'info, Event>,

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
        init,
        payer = better,
        space = 8 + Bet::INIT_SPACE,
        seeds = [
            b"bet",
            better.key().as_ref(),
            event.key().as_ref(),
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub bet: Account<'info, Bet>,
   

    #[account(
        mut,
        seeds = [
            b"user",
            better.key().as_ref(),
            user.seed.to_le_bytes().as_ref()
        ],
        bump = user.bump
    )]
    pub user: Account<'info, User>,

    #[account(
        init,
        payer = better,
        space = UserBet::INIT_SPACE,
        seeds = [
            b"user_bet",
            better.key().as_ref(),
            bet.key().as_ref(),
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub user_bet: Account<'info, UserBet>,


    #[account(
        mut,
        constraint = better_token_account.owner == better.key() @ ErrorCode::InvalidTokenAccount
    )]
    pub better_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = win_pool.key() == event.win_pool @ ErrorCode::InvalidWinPool
    )]
    pub win_pool: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}



impl<'info> PlaceBet<'info> {


    pub fn place_bet(
        &mut self,
        bet_amount: u64,
        outcome_index: u8,
        seed: u64,
    ) -> Result<()> {
        require!(outcome_index <= 1, ErrorCode::InvalidOutcomeIndex);
        require!(bet_amount > 0, ErrorCode::InvalidBetAmount);

        let better = self.better.key();
        let event = self.event.key();

        
        // We need to extract the shares values before any mutable borrows
        let outcome_yes_shares = self.outcome_yes.shares;
        let outcome_no_shares = self.outcome_no.shares;
        
        // Calculate shares using the extracted values
        let b: f64 = 100.0; // LMSR liquidity parameter (adjustable)
        let shares_before: f64 = (outcome_no_shares + outcome_yes_shares) as f64;
        let cost = b * ((shares_before + bet_amount as f64) / b).ln();
        let shares_bought = cost as u64;

        
        let outcome = if outcome_index == 1 {
            &mut self.outcome_yes
        } else {
            &mut self.outcome_no
        };


        // Transfer bet amount from better's token account to win_pool
        anchor_spl::token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: self.better_token_account.to_account_info(),
                    to: self.win_pool.to_account_info(),
                    authority: self.better.to_account_info(),
                },
            ),
            bet_amount,
        )?;

     
        // Update shares and total liquidity
        outcome.shares += shares_bought;
        outcome.total_liquidity += bet_amount;




        // Initialize bet account
        self.bet.set_inner(Bet {
            better,
            event,
            outcome: outcome.key(),
            outcome_index,
            claimed: false,
            creation_date: Clock::get()?.unix_timestamp,
            bet_amount,
            seed,
            bump: *self.bet.to_account_info().try_borrow_data()?.first().unwrap(),
        });

        // Initialize user_bet account
        self.user_bet.set_inner(UserBet {
            user: better,
            bet: self.bet.key(),
            seed,
            bump: *self.user_bet.to_account_info().try_borrow_data()?.first().unwrap(),
        });


        self.user.total_bets +=1;


        // Recalculate LMSR prices
        let (price_yes, price_no) =
        calculate_lmsr_price(self.outcome_yes.shares, self.outcome_no.shares);

        msg!("Updated Prices: Yes = {:.4}, No = {:.4}",price_yes,price_no);

         Ok(())

    }
}


