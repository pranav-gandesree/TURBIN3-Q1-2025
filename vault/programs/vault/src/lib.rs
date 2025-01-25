use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

declare_id!("F7v8dZYRXZhiUGXu8RySMSRPe49kZr1iXCPdSv2XQxpy");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    } 

    pub fn withdraw(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // our user who signs the transaction

    #[account(
        init, 
        space = VaultState :: INIT_SPACE,
        payer = user,
        seeds = [b"state",user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>, // this account stores the vault bump and state bump 

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, // this account actually stores the data 

    pub system_program: Program<'info, System>
}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        
        Ok(())
    }
}



//deposit struct for vault
#[derive(Accounts)]
pub struct Payments<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System> // bacause we are transferring sol, we use system_program here
}


impl<'info> Payments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {

        let cpi_program = self.system_program.to_account_info(); //target program

        let cpi_accounts = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext:: new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()>{  

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds = $[
            b"vault",
            self.vault_state.to_account_info().as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount);
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", signer.key.as_ref()],
        bump = vault_state.vault_bump,
        close = signer
    )]
    pub vault_state :  Account<'info,VaultState >,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info()
        };
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program, 
            cpi_accounts, 
            signer_seeds
        );

        let amount = self.vault.lamports();
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8
}

impl Space for VaultState{
    const INIT_SPACE: usize = 8 + 1 + 1; //8 for descriminator, 1 for vault_bump and state_bump
}