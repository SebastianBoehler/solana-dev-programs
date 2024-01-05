#![feature(build_hasher_simple_hash_one)]

use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("EfAx4Xm2XSfKy256BRALdW1EbR28LjvQErykasxCyp59");

#[program]
mod my_oracle {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        msg!("Init done for {}", name); // Message will show up in the tx log
        ctx.accounts.data_store.name = name;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        msg!("Update {}", data);
        ctx.accounts.data_store.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, payer = user, space = 8 + 8 + 4 + 7,
        seeds=[b"oracle", user.key().as_ref()], bump
    )]
    pub data_store: Account<'info, DataStore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"oracle", user.key().as_ref()], bump
    )]
    pub data_store: Account<'info, DataStore>,
}

#[account]
pub struct DataStore {
    pub name: String,
    pub data: u64,
}

