use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

    #[msg("Title exceeds 100 characters")]
    TitleTooLong,

    #[msg("Invalid outcome index")]
    InvalidOutcomeIndex,

    #[msg("Invalid outcome account")]
    InvalidOutcome,

    #[msg("Event already resolved")]
    EventAlreadyResolved,

    #[msg("Event not resolved yet")]
    EventNotResolved,

    #[msg("Reward already claimed")]
    AlreadyClaimed,

    #[msg("Unauthorized access")]
    UnauthorizedAccess,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Invalid win pool")]
    InvalidWinPool,

    #[msg("Invalid bet")]
    InvalidBet,
    
    #[msg("Invalid Switchboard account")]
    InvalidSwitchboardAccount,

    #[msg("Bet amount should be greater than zero")]
    InvalidBetAmount,
}


 