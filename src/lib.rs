// Copyright (c) 2021 Plutonium Network

pub mod sol_cancel;
pub mod sol_initialize;
pub mod sol_withdraw;
pub mod tok_initialize;
pub mod utils;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use sol_cancel::sol_cancel_stream;
use sol_initialize::sol_initialize_stream;
use sol_withdraw::sol_withdraw_unlocked;
use tok_initialize::tok_initialize_stream;

entrypoint!(process_instruction);
/// The program entrypoint
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
      PlutoniumNetwork  " v{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    match instruction_data[0] {
        // These are for native SOL
        0 => sol_initialize_stream(program_id, accounts, instruction_data),
        1 => sol_withdraw_unlocked(program_id, accounts, instruction_data),
        2 => sol_cancel_stream(program_id, accounts, instruction_data),
        // These are for SPL tokens
        3 => tok_initialize_stream(program_id, accounts, instruction_data),
        // 4 => tok_withdraw_unlocked(program_id, accounts, instruction_data),
        // 5 => tok_cancel_stream(program_id, accounts, instruction_data),
        // Invalid
        _ => Err(ProgramError::InvalidArgument),
    }
}
