use anchor_lang::prelude::*;

use crate::{Vault, DISCRIMINATOR, VAULT_SEED};

pub fn _initialize(_ctx: Context<InitializeAccounts>) -> Result<()> {
    msg!("Initializing vault");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault_state: Signer<'info>,
    #[account(
        seeds = [VAULT_SEED, vault_state.key().as_ref()],
        bump = vault.auth_bump
    )]
    pub vault_auth: AccountInfo<'info>,
    #[account(
        init,
        payer = owner,
        space = DISCRIMINATOR + Vault::INIT_SPACE,
        seeds = [VAULT_SEED, vault_auth.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}
