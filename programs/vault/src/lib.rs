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

    pub fn hello(ctx: Context<HelloAccounts>, input: HelloInput) -> Result<()> {
        _hello(ctx, input)
    }

    pub fn open(ctx: Context<OpenAccounts>, input: OpenInput) -> Result<()> {
        _open(ctx, input)
    }

    pub fn deposit(ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
        _deposit(ctx, input)
    }

    pub fn withdraw(ctx: Context<WithdrawAccounts>, input: WithdrawInput) -> Result<()> {
        _withdraw(ctx, input)
    }
}
