use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Vault, STATE_SEED, VAULT_SEED};

pub fn _deposit_spl(ctx: Context<DepositSplAccounts>, input: DepositSplInput) -> Result<()> {
    let from = ctx.accounts.owner_ata.to_account_info();
    let to = ctx.accounts.vault_ata.to_account_info();
    let mint = ctx.accounts.token_mint.to_account_info();
    let decimals = ctx.accounts.token_mint.decimals;

    let cpi_accounts = TransferChecked {
        authority: ctx.accounts.owner.to_account_info(),
        from,
        to,
        mint,
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_context, input.amount, decimals)
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
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
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
