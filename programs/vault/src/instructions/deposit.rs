use anchor_lang::{prelude::*, system_program};

use crate::{Vault, VAULT_SEED};

fn transfer_sol_to_vault(ctx: &Context<DepositAccounts>, input: &DepositInput) -> Result<()> {
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);

    system_program::transfer(cpi_ctx, input.amount)?;

    Ok(())
}

pub fn _deposit(ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.balance = vault.balance.checked_add(input.amount).unwrap();

    transfer_sol_to_vault(&ctx, &input)?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_SEED, payer.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: SystemAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositInput {
    pub amount: u64,
}
