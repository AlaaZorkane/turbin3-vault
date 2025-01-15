use anchor_lang::prelude::*;

#[error_code]
pub enum HelloErrors {
    #[msg("Invalid hello error")]
    InvalidHelloError,
}

#[error_code]
pub enum WithdrawErrors {
    #[msg("Insufficient lamports in vault")]
    InsufficientLamports,
    #[msg("Overflow")]
    Overflow,
}
