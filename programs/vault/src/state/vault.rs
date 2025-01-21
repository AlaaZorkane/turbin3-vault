use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub score: u8,
}
