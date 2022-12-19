use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn add(account_data: &mut Adder, number: &u32) -> ProgramResult {
    account_data.number += number;

    Ok(())
}
