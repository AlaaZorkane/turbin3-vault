use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Vault, VAULT_SEED};

pub fn _deposit_spl(_ctx: Context<DepositSplAccounts>, _input: DepositSplInput) -> Result<()> {
    msg!("Depositing SPL to vault");

    Ok(())
}

#[derive(Accounts)]
pub struct DepositSplAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
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
        seeds = [VAULT_SEED, vault_auth.key().as_ref()],
        bump = vault.vault_bump
    )]
    pub vault: Account<'info, Vault>,
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositSplInput {
    pub amount: u64,
}
