use crate::error::AdderError::InvalidInstruction;
use borsh::{BorshDeserialize};
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum AdderInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]`
    /// 1. `[writable]` The account that stores the accumulator
    /// 2. `[]` The rent sysvar
    /// 3. `[]` The system program
    Add { number: u32, bump: u8 },
    /// Accounts expected:
    ///
    /// 0. `[signer]` 
    /// 1. `[writable]` The account that stores the accumulator
    /// 2. `[]` The rent sysvar
    /// 3. `[]` The system program
    Double { bump: u8 },
}

#[derive(BorshDeserialize, Debug)]
struct Payload {
    variant: u8,
    bump: u8,
    number: u32,
}

impl AdderInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack");

        let payload = Payload::try_from_slice(input).unwrap();
        Ok(match payload.variant {
            0 => Self::Add {
                bump: payload.bump,
                number: payload.number,
            },
            1 => Self::Double { bump: payload.bump },

            _ => return Err(InvalidInstruction)?,
        })
    }
}
