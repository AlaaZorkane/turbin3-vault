use anchor_lang::prelude::*;

use crate::{Vault, VAULT_SEED};

pub fn _deposit(_ctx: Context<DepositAccounts>, _input: DepositInput) -> Result<()> {
    msg!("Depositing SOL to vault");

    Ok(())
}

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        owner = owner.key()
    )]
    pub vault_state: AccountInfo<'info>,
    #[account(
        seeds = [VAULT_SEED, vault_state.key().as_ref()],
        bump = vault.auth_bump
    )]
    pub vault_auth: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [VAULT_SEED, vault_auth.key().as_ref()],
        bump = vault.vault_bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositInput {
    pub amount: u64,
}
