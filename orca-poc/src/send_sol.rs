use orca_poc::get_wallet;
use solana_client::nonblocking::rpc_client::RpcClient; // Use non-blocking for async
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, message::Message, pubkey::Pubkey, signer::Signer,
    system_instruction, transaction::Transaction,
};
use std::{env, str::FromStr};

// https://solana.com/developers/cookbook/transactions/send-sol
// How to Send SOL
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

    // Get latest blockhash
    let latest_blockhash = rpc.get_latest_blockhash().await?;

    // Create transfer instruction
    let amount = 1_000_000; // Amount in lamports (0.001 SOL)
    let transfer_instruction = system_instruction::transfer(
        &signer.pubkey(), // Source
        &destination,
        amount,
    );

    // https://solana.com/developers/cookbook/transactions/calculate-cost
    // How to Calculate Transaction Cost
    // Step 1: Simulate to estimate compute units
    println!("Estimating the compute consumption of the transaction...");
    let message = Message::new(&[transfer_instruction.clone()], Some(&signer.pubkey()));
    let simulation_result = rpc
        .simulate_transaction(&Transaction::new_unsigned(message.clone()))
        .await?;
    let default_compute_units = 200; // Default to 200 if unavailable
    let mut estimated_compute_units = simulation_result
        .value
        .units_consumed
        .unwrap_or(default_compute_units);
    if estimated_compute_units == 0 {
        estimated_compute_units = default_compute_units; // Fallback to reasonable default
    }
    println!(
        "Transaction is estimated to consume {} compute units",
        estimated_compute_units
    );

    // Step 2: Set compute budget
    let compute_budget_instruction = ComputeBudgetInstruction::set_compute_unit_limit(
        estimated_compute_units as u32 + 100, // Add buffer
    );

    // Build transaction with compute budget
    let instructions = vec![compute_budget_instruction, transfer_instruction];
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&signer.pubkey()),
        &[&signer],
        latest_blockhash,
    );

    // Step 3: Calculate transaction fee
    let fee = rpc.get_fee_for_message(&tx.message).await?;
    println!("Transaction is estimated to cost {} lamports", fee);

    // Check balance
    let balance = rpc.get_balance(&signer.pubkey()).await?;
    println!("Wallet balance: {} lamports", balance);

    // Validate balance
    let required = amount + fee;
    if balance < required {
        return Err(format!(
            "Insufficient balance: {balance} lamports. Need at least {required} lamports"
        )
        .into());
    }

    // Send and confirm transaction
    let signature = rpc.send_and_confirm_transaction(&tx).await?;
    println!("Transaction sent! Signature: {}", signature);
    println!(
        "Scan: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    Ok(())
}
