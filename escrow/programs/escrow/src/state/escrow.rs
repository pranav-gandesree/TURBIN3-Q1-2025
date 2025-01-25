use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub seed: u64, // helps in maintaing more than one scrow, when seriving the pda of the user, seed and user key can be helpful
    pub maker: Pubkey, // public key of the maker
    pub mint_a: Pubkey, // public key of the token, maker want to send
    pub mint_b: Pubkey, //public key of the token, maker want to receive
    pub receive: u64, // amount, the maker want to receive 
    pub bump : u8
}

