use crate::id;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey, system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum TransferInstruction {
    /// Transfer sol tokens
    /// Accounts:
    /// 0. `[signer, writable]` who is transfering tokens
    /// 1. '[writable]' to whom tokens are transfered
    /// 2. '[]' System program which makes actual transfer of tokens
    Transfer {
        amount: u64
    },
}

impl TransferInstruction {
    pub fn transfer(sender: &Pubkey, receiver: &Pubkey, amount: u64) -> Instruction {
        Instruction::new_with_borsh(
            id(),
            &TransferInstruction::Transfer { amount },
            vec![
                AccountMeta::new(*sender, true),
                AccountMeta::new(*receiver, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ]
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let data = TransferInstruction::Transfer { amount: 10_000_000_000 }.try_to_vec().unwrap();
        assert_eq!(
            data, 
            [0, 0, 228, 11, 84, 2, 0, 0, 0]
        );
    }
}