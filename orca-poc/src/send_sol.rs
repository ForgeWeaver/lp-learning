use orca_poc::get_wallet;
use solana_client::nonblocking::rpc_client::RpcClient; // Use non-blocking for async
use solana_sdk::{pubkey::Pubkey, signer::Signer, system_instruction, transaction::Transaction};
use std::{env, str::FromStr};

// https://solana.com/developers/cookbook/transactions/send-sol
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Load keypair from default Solana CLI path
    let signer = get_wallet();
    println!("Signer public key: {}", signer.pubkey());

    // Create RPC client for Devnet (non-blocking)
    let rpc_url = "https://api.devnet.solana.com";
    let rpc = RpcClient::new(rpc_url.to_string());

    // Destination address from environment variable
    let destination_str = env::var("TEST_ACCOUNT_01").map_err(
        |_| "Environment variable TEST_ACCOUNT_01 not set. Please set it to a valid pubkey.",
    )?;
    let destination = Pubkey::from_str(&destination_str)?;

    // Check balance
    let balance = rpc.get_balance(&signer.pubkey()).await?;
    println!("Wallet balance: {} lamports", balance);
    if balance < 1_005_000 {
        // 0.001 SOL + ~0.000005 SOL fee
        return Err("Insufficient balance for transfer and fees".into());
    }

    // Get latest blockhash
    let latest_blockhash = rpc.get_latest_blockhash().await?;

    // Create transfer instruction
    let instruction = system_instruction::transfer(
        &signer.pubkey(), // Source
        &destination,     // Destination
        1_000_000,        // Amount in lamports (0.001 SOL)
    );

    // Build transaction
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer],
        latest_blockhash,
    );

    // Send and confirm transaction
    let signature = rpc.send_and_confirm_transaction(&tx).await?;
    println!("Transaction sent! Signature: {}", signature);
    println!(
        "Scan: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    Ok(())
}
