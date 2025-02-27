use solana_sdk::signer::Signer;
use solana_sdk::signer::keypair::Keypair;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_solana_keypair_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home_dir);
    path.push(".config/solana/id.json");
    path
}

// https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Environment%20Setup
fn main() {
    let keypair_path = get_solana_keypair_path();
    println!("Solana keypair path: {:?}", keypair_path);

    let wallet_string = fs::read_to_string(&keypair_path).unwrap();
    let keypair_bytes: Vec<u8> = serde_json::from_str(&wallet_string).unwrap();
    let wallet = Keypair::from_bytes(&keypair_bytes).unwrap();
    println!("Valid keypair found. Public key: {:?}", wallet.pubkey());
}
