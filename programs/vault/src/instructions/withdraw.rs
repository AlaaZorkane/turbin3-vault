use crate::{Vault, VAULT_SEED};
use anchor_lang::prelude::*;

pub fn _withdraw(_ctx: Context<WithdrawAccounts>, _input: WithdrawInput) -> Result<()> {
    msg!("Withdrawing SOL from vault");

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
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
pub struct WithdrawInput {
    pub amount: u64,
}
