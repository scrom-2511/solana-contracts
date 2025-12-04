use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{self, create_account},
    sysvar::Sysvar,
    entrypoint
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Info {
    name: String,
    age: u16,
    email: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum InstructionData {
    SetData { info: Info },
}
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let recieved_ix = InstructionData::try_from_slice(instruction_data)?;
    let account_data = &mut accounts.iter();
    let payer = next_account_info(account_data)?;
    let pda_name = next_account_info(account_data)?;
    let pda_age = next_account_info(account_data)?;
    let pda_email = next_account_info(account_data)?;
    let system_program = next_account_info(account_data)?;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let seed_name = &[b"name", payer.key.as_ref()];
    let seed_age = &[b"age", payer.key.as_ref()];
    let seed_email = &[b"email", payer.key.as_ref()];

    let (expected_pda_name, name_bump) = Pubkey::find_program_address(seed_name, program_id);
    if expected_pda_name != *pda_name.key {
        return Err(ProgramError::InvalidArgument);
    }

    let (expected_pda_age, age_bump) = Pubkey::find_program_address(seed_age, program_id);
    if expected_pda_age != *pda_age.key {
        return Err(ProgramError::InvalidArgument);
    }

    let (expected_pda_email, email_bump) = Pubkey::find_program_address(seed_email, program_id);
    if expected_pda_email != *pda_email.key {
        return Err(ProgramError::InvalidArgument);
    }

    let rent = Rent::get()?;
    let space = 50;
    let lamports = rent.minimum_balance(space);
    if pda_name.data_len() == 0 {
        let ix = create_account(
            payer.key,
            pda_name.key,
            lamports,
            space as u64,
            program_id,
        );

        invoke_signed(
            &ix,
            &[payer.clone(), pda_name.clone(), system_program.clone()],
            &[&[b"name", payer.key.as_ref(), &[name_bump]]],
        )?;
    }

    if pda_age.data_len() == 0 {
        let ix = create_account(
            payer.key,
            pda_age.key,
            lamports,
            space as u64,
            program_id,
        );
        invoke_signed(
            &ix,
            &[payer.clone(), pda_age.clone(), system_program.clone()],
            &[&[b"age", payer.key.as_ref(), &[age_bump]]],
        )?;
    }

    if pda_email.data_len() == 0 {
        let ix = create_account(
            payer.key,
            pda_email.key,
            lamports,
            space as u64,
            program_id,
        );
        invoke_signed(
            &ix,
            &[payer.clone(), pda_email.clone(), system_program.clone()],
            &[&[b"email", payer.key.as_ref(), &[email_bump]]],
        )?;
    }

    match recieved_ix {
        InstructionData::SetData { info } => {
            info.name
                .serialize(&mut &mut pda_name.data.borrow_mut()[..])?;
            info.age
                .serialize(&mut &mut pda_age.data.borrow_mut()[..])?;
            info.email
                .serialize(&mut &mut pda_email.data.borrow_mut()[..])?;
        }
    }

    Ok(())
}
