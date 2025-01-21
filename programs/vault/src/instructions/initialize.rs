use anchor_lang::prelude::*;

use crate::{Vault, DISCRIMINATOR, STATE_SEED, VAULT_SEED};

pub fn _initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
    msg!("Initializing vault");

    let vault = &mut ctx.accounts.state;
    vault.state_bump = ctx.bumps.state;
    vault.vault_bump = ctx.bumps.vault;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = DISCRIMINATOR + Vault::INIT_SPACE,
        seeds = [STATE_SEED, owner.key().as_ref()],
        bump
    )]
    pub state: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [VAULT_SEED, state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
