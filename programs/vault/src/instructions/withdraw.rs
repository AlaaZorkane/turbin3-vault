use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Vault, STATE_SEED, VAULT_SEED};

pub fn _withdraw(ctx: Context<WithdrawAccounts>, input: WithdrawInput) -> Result<()> {
    msg!("Withdrawing SOL from vault");

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

    transfer(cpi, input.amount)
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
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
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawInput {
    pub amount: u64,
}
