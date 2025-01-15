use crate::constants::*;
use anchor_lang::{prelude::*, system_program};

use crate::Vault;

fn transfer_sol_to_vault(ctx: &Context<OpenAccounts>, input: &OpenInput) -> Result<()> {
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi_ctx, input.initial_balance)?;

    Ok(())
}

pub fn _open(ctx: Context<OpenAccounts>, input: OpenInput) -> Result<()> {
    if input.initial_balance > 0 {
        transfer_sol_to_vault(&ctx, &input)?;
    }

    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.payer.key();
    vault.bump = ctx.bumps.vault;
    vault.balance = input.initial_balance;

    Ok(())
}

#[derive(Accounts)]
pub struct OpenAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = DISCRIMINATOR + Vault::INIT_SPACE,
        seeds = [VAULT_SEED, payer.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: SystemAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OpenInput {
    pub initial_balance: u64,
}
