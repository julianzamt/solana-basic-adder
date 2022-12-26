use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn ix_logic(accumulator: &mut Accumulator) -> ProgramResult {
    // Place business logic here
    accumulator.number *= 2;

    Ok(())
}
