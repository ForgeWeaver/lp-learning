use orca_poc::get_wallet;
use orca_whirlpools::{WhirlpoolsConfigInput, set_funder, set_whirlpools_config_address};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::Signer;

// https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Environment%20Setup
#[allow(dead_code)]
pub fn run() {
    // Wallet Management
    let wallet = get_wallet();

    // Configure the Whirlpools SDK for Your Network
    println!("Connecting to SolanaDevnet...");
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet).unwrap();

    // Airdrop SOL to Your Wallet
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    match rpc_client.request_airdrop(&wallet.pubkey(), 1_000_000_000) {
        Ok(signature) => println!("Airdrop successful. Transactoin signature: {:?}", signature),
        Err(e) => println!("Error: {:?}", e),
    }

    // Set the default funder for Transactions
    set_funder(wallet.pubkey()).unwrap();
}
