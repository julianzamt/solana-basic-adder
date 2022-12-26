use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::create_account,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::*, state::*, stubs::*};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = AdderInstruction::unpack(instruction_data)?;

        match instruction {
            AdderInstruction::Add { bump, number } => {
                msg!("Instruction: Add");
                Self::process_add(bump, number, accounts, program_id)?;
            }
            AdderInstruction::Double { bump } => {
                msg!("Instruction: Double");
                Self::process_double(bump, accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_add(
        bump: u8,
        number: u32,
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer_account_info = next_account_info(account_info_iter)?;
        let accumulator_account_info = next_account_info(account_info_iter)?;

        // Init if needed
        if accumulator_account_info.lamports() == 0 && *accumulator_account_info.owner == solana_program::system_program::id()
        {
            let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

            let space = 4;

            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer_account_info.key,
                    &accumulator_account_info.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer_account_info.clone(), accumulator_account_info.clone()],
                &[&[b"accumulator".as_ref(), &[bump]]],
            )?;
        }

        let mut accumulator_data = Accumulator::unpack_unchecked(&accumulator_account_info.try_borrow_data()?)?;

        msg!("Accumulator before add: {}", accumulator_data.number);

        add_stub::ix_logic(&mut accumulator_data, &number)?;

        Accumulator::pack(accumulator_data.clone(), &mut accumulator_account_info.try_borrow_mut_data()?)?;

        msg!("Accumulator after add: {}", accumulator_data.number);

        Ok(())
    }

    pub fn process_double(
        bump: u8,
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("process double");
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        let accumulator = next_account_info(account_info_iter)?;

        // Init if needed
        if accumulator.lamports() == 0 && *accumulator.owner == solana_program::system_program::id()
        {
            let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

            let space = 4;

            let rent = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &accumulator.key,
                    rent,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), accumulator.clone()],
                &[&[b"accumulator".as_ref(), &[bump]]],
            )?;
        }

        let mut accumulator_data = Accumulator::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        msg!(
            "Accumulator before double: {}",
            accumulator_data.number
        );

        double_stub::ix_logic(&mut accumulator_data)?;

        msg!(
            "Accumulator after double: {}",
            accumulator_data.number
        );

        Accumulator::pack(accumulator_data, &mut accumulator.try_borrow_mut_data()?)?;

        Ok(())
    }
}
