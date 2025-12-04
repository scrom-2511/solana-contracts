use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
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
    let _signer = next_account_info(account_info_iter)?;
    let data_account = next_account_info(account_info_iter)?;

    if data_account.owner != program_id {
        msg!("Data account is not owned by this program baby");
        return Err(ProgramError::IncorrectProgramId);
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

    let mut account_bytes = data_account.data.borrow_mut();
    let mut bytes_slice = &mut account_bytes[..];
    calculator.serialize(&mut bytes_slice)?;

    Ok(())
}
