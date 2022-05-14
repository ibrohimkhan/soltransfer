use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum TransferInstruction {
    /// Transfer sol tokens
    /// Accounts:
    /// 0. `[signer, writable]` who is transfering tokens
    /// 1. '[writable]' to whom tokens are transfered
    /// 2. '[]' System program which makes actual transfer of tokens
    Transfer { amount: u64 },
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::LAMPORTS;

    #[test]
    fn test_serialization() {
        let data = TransferInstruction::Transfer {
            amount: 10 * LAMPORTS,
        }
        .try_to_vec()
        .unwrap();
        
        assert_eq!(data, [0, 0, 228, 11, 84, 2, 0, 0, 0]);
    }
}
