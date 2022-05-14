pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;


solana_program::declare_id!("8NRM4Q5fkotVDGRWLSkMW1NMrtc8adRnHBCmFv4Q4H8t");
