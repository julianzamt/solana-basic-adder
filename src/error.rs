use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum AdderError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<AdderError> for ProgramError {
    fn from(e: AdderError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
