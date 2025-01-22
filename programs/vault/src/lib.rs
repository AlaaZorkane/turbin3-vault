#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("vLtGiWe6zK8rx3fuRXrm5er2EccAr4XpjssYNEJDLBH");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        _initialize(ctx)
    }

    pub fn deposit(ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
        _deposit(ctx, input)
    }

    pub fn withdraw(ctx: Context<WithdrawAccounts>, input: WithdrawInput) -> Result<()> {
        _withdraw(ctx, input)
    }

    pub fn deposit_spl(ctx: Context<DepositSplAccounts>, input: DepositSplInput) -> Result<()> {
        _deposit_spl(ctx, input)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSplAccounts>, input: WithdrawSplInput) -> Result<()> {
        _withdraw_spl(ctx, input)
    }

    pub fn close_account(ctx: Context<CloseAccounts>) -> Result<()> {
        _close_account(ctx)
    }
}
