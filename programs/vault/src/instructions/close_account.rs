use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Vault, STATE_SEED, VAULT_SEED};

pub fn _close_account(ctx: Context<CloseAccounts>) -> Result<()> {
    msg!("Closing account");

    let cpi_instruction = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();

    let state_key = ctx.accounts.state.key();
    let vault_bump = ctx.accounts.state.vault_bump;
    let seeds = &[VAULT_SEED, state_key.as_ref(), &[vault_bump]];
    let signature = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(cpi_program, cpi_instruction, signature);
    let amount = ctx.accounts.vault.lamports();

    transfer(cpi, amount)
}

#[derive(Accounts)]
pub struct CloseAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        close = owner,
        seeds = [STATE_SEED, owner.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [VAULT_SEED, state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
