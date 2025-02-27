use solana_sdk::signer::{Signer, keypair::Keypair};
use std::{env, fs, path::PathBuf};

// Wallet Management
pub fn get_wallet() -> Keypair {
    let keypair_path = get_solana_keypair_path();
    println!("Solana keypair path: {:?}", keypair_path);

    let wallet_string = fs::read_to_string(&keypair_path).unwrap();
    let keypair_bytes: Vec<u8> = serde_json::from_str(&wallet_string).unwrap();
    let wallet = Keypair::from_bytes(&keypair_bytes).unwrap();
    println!("Valid keypair found. Public key: {:?}", wallet.pubkey());

    wallet
}

fn get_solana_keypair_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home_dir);
    path.push(".config/solana/id.json");
    path
}
