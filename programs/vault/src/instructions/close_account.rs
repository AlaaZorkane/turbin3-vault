use anchor_lang::prelude::*;

pub fn _close_account(_ctx: Context<CloseAccounts>) -> Result<()> {
    msg!("Closing account");

    Ok(())
}

#[derive(Accounts)]
pub struct CloseAccounts<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        owner = owner.key()
    )]
    pub close_vault_state: AccountInfo<'info>,
    #[account(
        mut,
        owner = owner.key()
    )]
    pub vault_state: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
