use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::transfer_checked;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let from_token_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let to_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !from.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction = transfer_checked(
        &token_program.key,
        &from_token_account.key,
        &mint_account.key,
        to_token_account.key,
        &from.key,
        &[&from.key],
        1000000000,
        9,
    );

    msg!("Calling the token program to transfer tokens...");

    invoke(
        &instruction.unwrap(),
        &[
            from_token_account.clone(),
            mint_account.clone(),
            to_token_account.clone(),
            from.clone(),
        ],
    )?;

    Ok(())
}
