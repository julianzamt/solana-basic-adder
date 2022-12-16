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

use crate::{instruction::*, state::*};

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

        let signer = next_account_info(account_info_iter)?;
        let accumulator = next_account_info(account_info_iter)?;

        // Init if needed
        if accumulator.lamports() == 0 && *accumulator.owner == solana_program::system_program::id()
        {
            let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

            let space = 4;

            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &accumulator.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), accumulator.clone()],
                &[&[b"accumulator".as_ref(), &[bump]]],
            )?;
        }

        let mut accumulator_data = Adder::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        msg!("accumulator number: {}", accumulator_data.number);
        msg!("number: {}", number);

        accumulator_data.number += number;

        Adder::pack(accumulator_data, &mut accumulator.try_borrow_mut_data()?)?;

        let accumulator_data = Adder::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        msg!(
            "accumulator number after stored add: {}",
            accumulator_data.number
        );

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

        let mut accumulator_data = Adder::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        msg!(
            "accumulator number before double: {}",
            accumulator_data.number
        );

        accumulator_data.number *= 2;

        msg!(
            "accumulator number after double: {}",
            accumulator_data.number
        );

        Adder::pack(accumulator_data, &mut accumulator.try_borrow_mut_data()?)?;

        let accumulator_data = Adder::unpack_unchecked(&accumulator.try_borrow_data()?)?;

        msg!(
            "accumulator number double after storage: {}",
            accumulator_data.number
        );

        Ok(())
    }
}
