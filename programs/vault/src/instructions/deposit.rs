use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Vault, STATE_SEED, VAULT_SEED};

pub fn _deposit(ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
    msg!("Depositing SOL to vault");

    let cpi_instruction = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi = CpiContext::new(cpi_program, cpi_instruction);

    transfer(cpi, input.amount)
}

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
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
pub struct DepositInput {
    pub amount: u64,
}
