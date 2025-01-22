use crate::{constants::ANCHOR_DESCRIMINATOR, state::Offer};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: 
}