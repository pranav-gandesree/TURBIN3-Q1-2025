use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a : InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token:: authority = maker
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, 
        payer = maker,
        space = Escrow::INIT_SPACE + 8,
        seeds = [b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,//escrow to store data 

    #[account(
        init,
        payer= maker,
        associated_token:: mint = mint_a,
        associated_token:: authority = maker
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,

}

impl<'info> Make<'info>{
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps)-> Result<()>{
        self.escrow.set_inner(Escrow{
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        });

        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = TransferChecked{
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info()
        };

        let cpi_ctx = CpiContext:: new(cpi_program, cpi_account);

        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;

        Ok(())

    }
}