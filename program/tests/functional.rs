#![cfg(feature = "test-bpf")]

use solana_program::system_instruction;
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use soltransfer::{entrypoint::process_instruction, id};

struct Env {
    ctx: ProgramTestContext,
    sender: Keypair,
    receiver: Keypair,
}

impl Env {
    async fn new() -> Self {
        let program_test = ProgramTest::new("soltransfer", id(), processor!(process_instruction));
        let mut ctx = program_test.start_with_context().await;

        let sender = Keypair::new();
        let receiver = Keypair::new();

        // credit sender account
        ctx.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &sender.pubkey(),
                    10_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            ))
            .await
            .unwrap();

        Env {
            ctx,
            sender,
            receiver,
        }
    }
}

#[tokio::test]
async fn test_transfer() {
    let mut env = Env::new().await;

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &env.sender.pubkey(),
            &env.receiver.pubkey(),
            3_000_000_000,
        )],
        Some(&env.sender.pubkey()),
        &[&env.sender],
        env.ctx.last_blockhash,
    );

    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    let sender_account = env
        .ctx
        .banks_client
        .get_account(env.sender.pubkey())
        .await
        .expect("get_account")
        .expect("sender account not found");

    let sender_lamports = sender_account.lamports;

    let receiver_account = env
        .ctx
        .banks_client
        .get_account(env.receiver.pubkey())
        .await
        .expect("get_account")
        .expect("sender account not found");

    let receiver_lamports = receiver_account.lamports;

    assert_eq!(sender_lamports, 6999995000);
    assert_eq!(receiver_lamports, 3000000000);
}
