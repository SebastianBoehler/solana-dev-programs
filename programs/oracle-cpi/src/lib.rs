#![feature(build_hasher_simple_hash_one)]

use anchor_lang::prelude::*;
use my_oracle::program::MyOracle;
use my_oracle;

declare_id!("3ideNGLqtUS9ZWMDs1qcqs3wMiV6WQ7s6vFXmPji5hbA");

#[program]
mod oracle_cpi {
    use super::*;
    pub fn pull_data(ctx: Context<PullData>) -> Result<()> {
        //console log the data from DataStore
        let data = &ctx.accounts.oracle_pda;
        msg!("Data: {} {}", data.name, data.data);
        Ok(())
    }

    pub fn push_data(ctx: Context<PushData>, price: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.oracle_program.to_account_info(), 
            my_oracle::cpi::accounts::Update {
                data_store: ctx.accounts.oracle_pda.to_account_info(),
                user: ctx.accounts.signer.to_account_info(),
            },
        );
        my_oracle::cpi::update(
            cpi_context,
            price
        )?;
        msg!("Data pushed {}", price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PushData<'info> {
    #[account(mut)]
    pub oracle_pda: Account<'info, my_oracle::DataStore>,
    pub oracle_program: Program<'info, MyOracle>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct PullData<'info> {
    #[account(mut)]
    pub oracle_pda: Account<'info, my_oracle::DataStore>,
    //pub oracle_program: Program<'info, MyOracle>,
}
