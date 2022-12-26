use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn ix_logic(accumulator: &mut Accumulator, number: &u32) -> ProgramResult {
    // Place business logic here
    accumulator.number += number;

    Ok(())
}
