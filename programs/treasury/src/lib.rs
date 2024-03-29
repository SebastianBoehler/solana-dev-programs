use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use anchor_spl::token::Mint;

declare_id!("5hFG4V19ktLKzKjUAxcBs1dNBv5UFQPDsfVfgNvLWBAe");

#[program]
pub mod treasury {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, identifier: u64, amount: u64, pda_bump: u8) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_pda;
        escrow.identifier = identifier;
        escrow.user_sending = ctx.accounts.user_sending.key().clone();
        escrow.user_receiving = ctx.accounts.user_receiving.key().clone();
        escrow.token_mint = ctx.accounts.mint_of_token_being_sent.key().clone();
        escrow.amount = amount;

        let transfer_instruction = Transfer{
            from: ctx.accounts.users_sending_token_account.to_account_info(),
            to: ctx.accounts.escrow_wallet.to_account_info(),
            authority: ctx.accounts.user_sending.to_account_info(),
        };

        //seeds generation
        //let mint_of_token_being_sent_pk = ctx.accounts.mint_of_token_being_sent.key().clone();
        //let identifier_bytes = identifier.to_le_bytes();
        let bump_bytes = pda_bump.to_le_bytes();
        let inner = vec![
            b"state".as_ref(),
            ctx.accounts.user_sending.key.as_ref(),
            ctx.accounts.user_receiving.key.as_ref(),
            //mint_of_token_being_sent_pk.as_ref(), 
            //identifier_bytes.as_ref(),
            bump_bytes.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        // The `?` at the end will cause the function to return early in case of an error.
        // This pattern is common in Rust.
        anchor_spl::token::transfer(cpi_ctx, escrow.amount)?;

        msg!("Escrow initialized! {}", amount);

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, pda_bump: u8) -> Result<()> {
        let escrow = &ctx.accounts.escrow_pda;
        let amount = escrow.amount;

        if amount == 0 {
            return Err(ErrorCode::EscrowEmpty.into());
        }

        let transfer_instruction = Transfer{
            from: ctx.accounts.escrow_wallet.to_account_info(),
            to: ctx.accounts.users_receiving_token_account.to_account_info(),
            authority: ctx.accounts.escrow_pda.to_account_info(),
        };

        //let identifier_bytes = identifier.to_le_bytes();
        let bump_bytes = pda_bump.to_le_bytes();

        let inner = vec![
            b"state".as_ref(),
            ctx.accounts.user_sending.key.as_ref(),
            ctx.accounts.user_receiving.key.as_ref(),
            //mint_of_token_being_sent_pk.as_ref(), 
            //identifier_bytes.as_ref(),
            bump_bytes.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("Funds withdrawed out! {}", amount);

        //set escrow amount to 0
        *&mut ctx.accounts.escrow_pda.amount = 0;

        Ok(())
    }

    pub fn pull_back(ctx: Context<PullBackInstruction>, _identifier: u64, pda_bump: u8) -> Result<()> {
        let escrow = &ctx.accounts.escrow_pda;
        let amount = escrow.amount;

        if amount == 0 {
            return Err(ErrorCode::EscrowEmpty.into());
        }

        let transfer_instruction = Transfer{
            from: ctx.accounts.escrow_wallet.to_account_info(),
            to: ctx.accounts.users_sending_token_account.to_account_info(),
            authority: ctx.accounts.escrow_pda.to_account_info(),
        };

        //let identifier_bytes = identifier.to_le_bytes();
        let bump_bytes = pda_bump.to_le_bytes();
        let inner = vec![
            b"state".as_ref(),
            ctx.accounts.user_sending.key.as_ref(),
            ctx.accounts.user_receiving.key.as_ref(),
            //mint_of_token_being_sent_pk.as_ref(), 
            //identifier_bytes.as_ref(),
            bump_bytes.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("Funds transferred out! {}", amount);

        //set escrow amount to 0
        *&mut ctx.accounts.escrow_pda.amount = 0;

        Ok(())
    }

    pub fn close_escrow(ctx: Context<CloseEscrow>) -> Result<()> {
        let escrow = &ctx.accounts.escrow_pda;
        let amount = escrow.amount;

        if amount != 0 {
            return Err(ErrorCode::EscrowNotEmpty.into());
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(identifier: u64, pda_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    user_sending: Signer<'info>,   
    /// CHECK: This is not a signer account
    user_receiving: AccountInfo<'info>,
    //#[account(constraint = mint_of_token_being_sent.key() == escrow_wallet.mint)]
    mint_of_token_being_sent: Account<'info, Mint>,
    
    #[account(
        init,
        //space = 8 + u64 + pubkey + pubkey + pubkey + u64 + u8 + enum
        space = 8 + 8 + 32 + 32 + 32 + 8 + 1,
        payer = user_sending,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key().as_ref()],
        bump,
    )]
    escrow_pda: Account<'info, State>,

    #[account(
        init,
        payer = user_sending,
        seeds=[b"wallet".as_ref(), user_sending.key().as_ref(), user_receiving.key().as_ref()],
        bump,
        token::mint = mint_of_token_being_sent,
        token::authority = escrow_pda,
    )]
    escrow_wallet: Account<'info, TokenAccount>,

    // Users associated token account
    #[account(
        mut,
        constraint=users_sending_token_account.owner == user_sending.key(),
        constraint=users_sending_token_account.mint == mint_of_token_being_sent.key()
    )]
    users_sending_token_account: Account<'info, TokenAccount>,

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default)]
pub struct State {
    identifier: u64,
    pub user_sending: Pubkey,
    pub user_receiving: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(identifier: u64, pda_bump: u8)]
pub struct PullBackInstruction<'info> {
    #[account(
        mut,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref()],
        bump,
        has_one = user_sending,
        has_one = user_receiving,
    )]
    escrow_pda: Account<'info, State>,
    #[account(
        mut,
        seeds=[b"wallet".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref()],
        bump,
    )]
    escrow_wallet: Account<'info, TokenAccount>,    
    // Users and accounts in the system
    #[account(mut)]
    user_sending: Signer<'info>,
    /// CHECK: This is not a signer account
    user_receiving: AccountInfo<'info>,
    mint_of_token_being_sent: Account<'info, Mint>,

    // Wallet to deposit to
    #[account(
        mut,
        constraint=users_sending_token_account.owner == user_sending.key(),
        constraint=users_sending_token_account.mint == mint_of_token_being_sent.key()
    )]
    users_sending_token_account: Account<'info, TokenAccount>,

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(pda_bump: u8)]
pub struct Withdraw<'info> {
    /// CHECK: This is not a signer account
    user_sending: AccountInfo<'info>,
    #[account(mut)]
    user_receiving: Signer<'info>,

    #[account(
        mut,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref()],
        bump,
        has_one = user_sending,
        has_one = user_receiving,
    )]
    escrow_pda: Account<'info, State>,

    #[account(
        mut,
        seeds=[b"wallet".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref()],
        bump,
    )]
    escrow_wallet: Account<'info, TokenAccount>,
    mint_of_token_being_sent: Account<'info, Mint>,

    #[account(
        mut,
        constraint=users_receiving_token_account.owner == user_receiving.key(),
        constraint=users_receiving_token_account.mint == mint_of_token_being_sent.key(),
    )]
    users_receiving_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CloseEscrow<'info> {
    user_sending: Signer<'info>,
    /// CHECK: This is not a signer account
    user_receiving: AccountInfo<'info>,
    #[account(
        mut,
        seeds=[b"state".as_ref(), user_sending.key().as_ref(), user_receiving.key.as_ref()],
        bump,
        close = user_sending,
    )]
    escrow_pda: Account<'info, State>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Escrow is not empty")]
    EscrowNotEmpty,
    #[msg("Escrow is empty")]
    EscrowEmpty,
}
