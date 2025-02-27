use orca_poc::get_wallet;
use orca_whirlpools::{
    WhirlpoolsConfigInput, create_concentrated_liquidity_pool_instructions,
    set_whirlpools_config_address,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction};
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creating Concentrated Liquidity Pools
    // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Create%20Pool#creating-concentrated-liquidity-pools

    // Configure for Devnet
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet)?;
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Token mints
    let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112")?; // Wrapped SOL
    let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k")?; // devUSDC
    let tick_spacing = 64;
    let initial_price = Some(0.01);
    let wallet = get_wallet();
    let funder = Some(wallet.pubkey());

    // Check balance
    let balance = rpc.get_balance(&wallet.pubkey()).await?;
    println!("Wallet balance: {} lamports", balance);

    // Generate instructions
    let result = create_concentrated_liquidity_pool_instructions(
        &rpc,
        token_a,
        token_b,
        tick_spacing,
        initial_price,
        funder,
    )
    .await?;

    println!("Pool Address: {:?}", result.pool_address);
    println!(
        "Scan: https://explorer.solana.com/address/{:?}?cluster=devnet",
        result.pool_address
    );
    println!(
        "Initialisation Cost: {} lamports",
        result.initialization_cost
    );

    // Check if pool exists
    let pool_info = rpc.get_account(&result.pool_address).await;
    if pool_info.is_ok() {
        println!("Pool already exists, skipping creation");
        return Ok(());
    }

    // Ensure sufficient balance
    if balance < result.initialization_cost {
        return Err(format!(
            "Insufficient balance: {} lamports. Need {}",
            balance, result.initialization_cost
        )
        .into());
    }

    // Debug: Print instruction details
    println!("Instructions to execute: {}", result.instructions.len());
    for (i, instr) in result.instructions.iter().enumerate() {
        println!("Instruction {}: Program ID: {}", i, instr.program_id);
        println!("  Accounts: {:?}", instr.accounts);
        println!("  Data length: {} bytes", instr.data.len());
        for account in &instr.accounts {
            if account.is_signer && account.pubkey != wallet.pubkey() {
                println!("Warning: Instruction requires signer: {}", account.pubkey);
                // Check if it's a PDA by attempting to derive it (optional validation)
                let (pda, _bump) = Pubkey::find_program_address(
                    &[result.pool_address.as_ref()], // Example seed, adjust as needed
                    &instr.program_id,
                );
                if pda == account.pubkey {
                    println!("  - Detected as PDA, should be signed by program");
                } else {
                    println!("  - Unknown signer, may need additional keypair");
                }
            }
        }
    }

    // Adjust instructions to remove erroneous signer flags
    let mut adjusted_instructions = result.instructions.clone();
    for instr in &mut adjusted_instructions {
        for account in &mut instr.accounts {
            if account.is_signer && account.pubkey != wallet.pubkey() {
                println!("Adjusting {}: Setting is_signer to false", account.pubkey);
                account.is_signer = false; // PDAs shouldn't need signing
            }
        }
    }

    // Build transaction
    let latest_blockhash = rpc.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &adjusted_instructions,
        Some(&wallet.pubkey()),
        &[&wallet],
        latest_blockhash,
    );

    // Transaction simulation failed: Error processing Instruction 0: Cross-program invocation with unauthorized signer or writable account
    // Send and confirm transaction
    let signature = rpc.send_and_confirm_transaction(&tx).await?;
    println!("Pool created! Transaction signature: {}", signature);
    println!(
        "Scan: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    Ok(())
}
