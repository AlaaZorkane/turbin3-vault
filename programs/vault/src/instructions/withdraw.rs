use anchor_lang::prelude::*;

use crate::{Vault, WithdrawErrors, VAULT_SEED};

fn transfer_sol_to_depositor<'info>(
    vault: &mut Account<'info, Vault>,
    depositor: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    let vault_account_lamps = vault.get_lamports();

    require!(
        vault_account_lamps >= amount,
        WithdrawErrors::InsufficientLamports
    );

    vault.sub_lamports(amount)?;
    depositor.add_lamports(amount)?;

    Ok(())
}

pub fn _withdraw(ctx: Context<WithdrawAccounts>, input: WithdrawInput) -> Result<()> {
    let amount = input.amount;
    let vault = &mut ctx.accounts.vault;

    transfer_sol_to_depositor(vault, &ctx.accounts.payer, amount)?;

    vault.balance = match vault.balance.checked_sub(amount) {
        Some(value) => value,
        None => return Err(WithdrawErrors::Overflow.into()),
    };

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
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
pub struct WithdrawInput {
    pub amount: u64,
}
