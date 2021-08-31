// Copyright (c) 2021 Plutonium Network


use std::{str::FromStr, time::SystemTime};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account, signature::Signer, signer::keypair::Keypair, system_program,
    transaction::Transaction,
};

use plutonium::process_instruction;
use plutonium::utils::{any_as_u8_slice, Plutonium};

#[repr(packed(1))]
struct StFl {
    instruction: u8,
    start_time: u64,
    end_time: u64,
    amount: u64,
}

#[tokio::test]
async fn test_initialize_stream() {
    let program_id = Pubkey::from_str(&"plutonium111111111111111111111111111111111").unwrap();

    let alice = Keypair::new();
    let bob = Keypair::new();
    let pda = Keypair::new();

    let mut program_test =
        ProgramTest::new("plutonium", program_id, processor!(process_instruction));

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let sf = StFl {
        instruction: 0,
        start_time: now as u64 + 10,
        end_time: now as u64 + 20,
        amount: sol_to_lamports(90.0),
    };

    println!("instruction: {}", { sf.instruction });
    println!("start_time: {}", { sf.start_time });
    println!("end_time: {}", { sf.end_time });
    println!("amount: {}", { sf.amount });

    let dat = Plutonium {
        start_time: now as u64 + 10,
        end_time: now as u64 + 20,
        amount: sol_to_lamports(90.0),
        withdrawn: 0,
        sender: alice.pubkey().to_bytes(),
        recipient: bob.pubkey().to_bytes(),
        mint: bob.pubkey().to_bytes(),   // placeholder
        escrow: bob.pubkey().to_bytes(), // placeholder
    };

    program_test.add_account(
        alice.pubkey(),
        Account {
            lamports: sol_to_lamports(1000.0),
            ..Account::default()
        },
    );

    program_test.add_account(
        pda.pubkey(),
        Account {
            lamports: 0,
            data: unsafe { any_as_u8_slice(&dat).to_vec() },
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            unsafe { any_as_u8_slice(&sf) },
            vec![
                AccountMeta::new(alice.pubkey(), true),
                AccountMeta::new(bob.pubkey(), false),
                AccountMeta::new(pda.pubkey(), true),
                AccountMeta::new(bob.pubkey(), false), // placeholder
                AccountMeta::new(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &alice, &pda], recent_blockhash);

    match banks_client.process_transaction(transaction).await {
        Ok(()) => (),
        Err(e) => panic!("{}", e),
    }

    // TODO: Asserts
}
