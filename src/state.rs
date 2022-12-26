use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, Clone, Copy, BorshDeserialize, BorshSerialize)]
pub struct Accumulator {
    pub number: u32,
}

impl Sealed for Accumulator {}

impl Pack for Accumulator {
    const LEN: usize = 4;
    fn unpack_from_slice(mut src: &[u8]) -> Result<Self, ProgramError> {
        let accumulator = Accumulator::deserialize(&mut src).unwrap();
        Ok(accumulator)
    }

    fn pack_into_slice(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap();
    }
}
