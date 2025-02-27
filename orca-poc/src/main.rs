use orca_whirlpools::{WhirlpoolsConfigInput, set_funder, set_whirlpools_config_address};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::{Signer, keypair::Keypair};
use std::{env, fs, path::PathBuf};

fn get_solana_keypair_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home_dir);
    path.push(".config/solana/id.json");
    path
}

// https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Environment%20Setup
fn main() {
    // Wallet Management
    let keypair_path = get_solana_keypair_path();
    println!("Solana keypair path: {:?}", keypair_path);

    let wallet_string = fs::read_to_string(&keypair_path).unwrap();
    let keypair_bytes: Vec<u8> = serde_json::from_str(&wallet_string).unwrap();
    let wallet = Keypair::from_bytes(&keypair_bytes).unwrap();
    println!("Valid keypair found. Public key: {:?}", wallet.pubkey());

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
