use anchor_lang::prelude::*;

use crate::{Vault, VAULT_SEED};

fn drain_sol_to_depositor<'info>(
    vault: &mut Account<'info, Vault>,
    depositor: &Signer<'info>,
) -> Result<()> {
    let all_lamps = vault.get_lamports();

    vault.sub_lamports(all_lamps)?;
    depositor.add_lamports(all_lamps)?;

    Ok(())
}

pub fn _close(ctx: Context<CloseAccounts>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    drain_sol_to_depositor(vault, &ctx.accounts.payer)?;

    vault.close(ctx.accounts.payer.to_account_info())?;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseAccounts<'info> {
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
