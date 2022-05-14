use borsh::BorshDeserialize;
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, pubkey::Pubkey, program::invoke, system_instruction};

use crate::instruction::TransferInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instructions_data: &[u8],
    ) -> ProgramResult {
        msg!("instructions_data: {:?}", instructions_data);
        let instruction = TransferInstruction::try_from_slice(instructions_data)?;

        match instruction {
            TransferInstruction::Transfer { amount } => Self::process_token_transfer(accounts, amount),
        }
    }

    fn process_token_transfer(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        msg!("process token transfer");
        let acc_iter = &mut accounts.iter();

        // Accounts
        let sender = next_account_info(acc_iter)?;
        let receiver = next_account_info(acc_iter)?;
        // the third account is SystemProgram. Don't forget it in a client

        invoke(
            &system_instruction::transfer(sender.key, receiver.key, amount),
            &[sender.clone(), receiver.clone()],
        )?;

        Ok(())
    }
}
