use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    error::ErrorCode,
    Bet,
    Event,
    Outcome, User, UserBet
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
        constraint = 
            (outcome_index == 0 && outcome.key() == event.outcomes[0]) ||
            (outcome_index == 1 && outcome.key() == event.outcomes[1])
            @ ErrorCode::InvalidOutcome
    )]
    pub outcome: Account<'info, Outcome>,

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