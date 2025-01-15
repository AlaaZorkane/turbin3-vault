use anchor_lang::prelude::*;

pub fn _hello(_ctx: Context<HelloAccounts>, input: HelloInput) -> Result<()> {
    msg!("Hello, world! {}", input.id);

    Ok(())
}

#[derive(Accounts)]
pub struct HelloAccounts<'info> {
    pub system_program: SystemAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HelloInput {
    pub id: u8,
}
