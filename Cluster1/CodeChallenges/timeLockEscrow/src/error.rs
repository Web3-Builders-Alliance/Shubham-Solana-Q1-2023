use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not rent exempt
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Invalid amount")]
    ExpectedAmountMismatch,
    #[error("Amount is too big")]
    AmountOverflow,
    #[error("Cannot exchange before unlock time")]
    InvalidUnlockTime,
    #[error("Cannot exchange after time out")]
    InvalidTimeOut,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
