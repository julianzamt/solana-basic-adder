use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

use arrayref::{array_mut_ref, array_ref};

pub struct Adder {
    pub number: u32,
}

impl Sealed for Adder {}

impl Pack for Adder {
    const LEN: usize = 4;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Adder::LEN];

        let number = u32::from_le_bytes(*src);
        Ok(Adder { number })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Adder::LEN];

        let Adder { number } = self;

        // FIXME -- serialization should work with a oneliner
        dst[0] = number.to_le_bytes()[0];
        dst[1] = number.to_le_bytes()[1];
        dst[2] = number.to_le_bytes()[2];
        dst[3] = number.to_le_bytes()[3];
    }
}
