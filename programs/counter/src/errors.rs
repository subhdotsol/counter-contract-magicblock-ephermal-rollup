use anchor_lang::prelude::*;

#[error_code]
pub enum CounterError {
    #[msg("Counter overflowed")]
    Overflow,

    #[msg("Invalid PDA")]
    InvalidPDA,
}
