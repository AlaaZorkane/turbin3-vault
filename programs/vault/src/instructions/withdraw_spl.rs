use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Vault, VAULT_SEED};

pub fn _withdraw_spl(_ctx: Context<WithdrawSplAccounts>, _input: WithdrawSplInput) -> Result<()> {
    // let vault = &mut ctx.accounts.vault;
    // vault.balance = vault.balance.checked_add(input.amount).unwrap();

    // transfer_sol_to_vault(&ctx, &input)?;

    msg!("Withdrawing SPL from vault");

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSplAccounts<'info> {
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
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault_auth,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawSplInput {
    pub amount: u64,
}
