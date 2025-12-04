use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},entrypoint, entrypoint::{ProgramResult}, msg, program::invoke, pubkey::Pubkey, system_instruction::create_account, sysvar::{Sysvar, rent::Rent}
};

entrypoint!(calculator_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorStruct {
    pub val: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CalculatorEnum {
    Init,
    Add { num: i32 },
    Subtract { num: i32 },
    Multiply { num: i32 },
    Divide { num: i32 },
}

pub fn calculator_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = CalculatorEnum::try_from_slice(instruction_data)?;

    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let data_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // 1️⃣ If data account does not exist, create it
    if data_account.owner != program_id {
        msg!("Creating data account...");

        let space = 4; // size of CalculatorStruct (one i32)
        let rent = Rent::get()?.minimum_balance(space);

        // Create account
        let create_ix = create_account(
            payer.key,
            data_account.key,
            rent,
            space as u64,
            program_id,
        );

        invoke(&create_ix, accounts)?;

        msg!("Data account created!");
    }

    let mut calculator = CalculatorStruct::try_from_slice(&data_account.data.borrow())
        .unwrap_or(CalculatorStruct { val: 0 });

    match ix {
        CalculatorEnum::Init => calculator.val = 0,
        CalculatorEnum::Add { num } => calculator.val += num,
        CalculatorEnum::Subtract { num } => calculator.val -= num,
        CalculatorEnum::Multiply { num } => calculator.val *= num,
        CalculatorEnum::Divide { num } => calculator.val /= num,
    }

    calculator.serialize(&mut &mut data_account.data.borrow_mut()[..])?;

    Ok(())
}
