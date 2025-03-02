use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount};

use crate::{
    calculate_lmsr_price, error::ErrorCode, Bet, Event, Outcome
};



#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(
        constraint = event.resolved @ ErrorCode::EventNotResolved
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        constraint = bet.better == better.key() @ ErrorCode::UnauthorizedAccess,
        constraint = !bet.claimed @ ErrorCode::AlreadyClaimed,
        constraint = bet.event == event.key() @ ErrorCode::InvalidBet
    )]
    pub bet: Account<'info, Bet>,

    #[account(mut)]
    pub better: Signer<'info>,

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

    #[account(
        mut,
        constraint = outcome.key() == event.outcomes[bet.outcome_index as usize] @ ErrorCode::InvalidOutcome
    )]
    pub outcome: Account<'info, Outcome>,


    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>
}




impl<'info> ClaimReward<'info> {

    pub fn claim_reward(&mut self) -> Result<()> {

        // Verify the bet was placed on the winning outcome
        if !self.event.resolved {
            return Err(ErrorCode::EventNotResolved.into());
        }
        
        match self.event.winning_outcome {
            Some(index) if index == self.bet.outcome_index => {
                // Bet is a winner, proceed with rewards
            }
            _ => return Err(ErrorCode::NotAWinner.into()),
        }

        let reward_amount = self.calculate_payout()?;


        anchor_spl::token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: self.win_pool.to_account_info(),
                    to: self.better_token_account.to_account_info(),
                    authority: self.event.to_account_info(),
                },
            ),
            reward_amount,
        )?;


        // Mark the bet as claimed
        self.bet.claimed = true;

        msg!(
            "Bet {} claimed. Transferred {} tokens to {}.",
            self.bet.key(),
            reward_amount,
            self.better.key()
        );

        Ok(())
    }



    fn calculate_payout(&self) -> Result<u64> {
        let (price_yes, _) =
            calculate_lmsr_price(self.outcome.shares, self.outcome.total_liquidity);

        let shares_bought = self.bet.bet_amount; // Assuming bet amount equals shares in LMSR

        let reward = (shares_bought as f64 * price_yes) as u64;

        Ok(reward)
    }

}