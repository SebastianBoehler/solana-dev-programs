#![feature(build_hasher_simple_hash_one)]

use anchor_lang::prelude::*;
use my_oracle::DataStore;

declare_id!("3ideNGLqtUS9ZWMDs1qcqs3wMiV6WQ7s6vFXmPji5hbA");

#[program]
mod oracle_master {
    use super::*;
    pub fn pull_data(ctx: Context<PullData>) -> Result<()> {
        //console log the data from DataStore
        msg!("Data: {}", ctx.accounts.oracle_pda.name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullData<'info> {
    #[account(mut)]
    pub oracle_pda: Account<'info, DataStore>,
    //pub oracle_program: Program<'info, MyOracle>,
}
