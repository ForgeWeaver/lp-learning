use orca_poc::get_wallet;
use orca_whirlpools::{
    WhirlpoolsConfigInput, create_concentrated_liquidity_pool_instructions,
    set_whirlpools_config_address,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signer};
use std::str::FromStr;
use tokio;

#[tokio::main]
async fn main() {
    // Creating Concentrated Liquidity Pools
    // https://dev.orca.so/Whirlpools%20SDKs/Whirlpools/Whirlpool%20Management/Create%20Pool#creating-concentrated-liquidity-pools
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaDevnet).unwrap();
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    let token_a = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let token_b = Pubkey::from_str("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k").unwrap(); // devUSDC
    let tick_spacing = 64;
    let initial_price = Some(0.01);
    let wallet = get_wallet();
    let funder = Some(wallet.pubkey());

    let result = create_concentrated_liquidity_pool_instructions(
        &rpc,
        token_a,
        token_b,
        tick_spacing,
        initial_price,
        funder,
    )
    .await
    .unwrap();

    println!("Pool Address: {:?}", result.pool_address);
    println!(
        "Scan: https://explorer.solana.com/address/{:?}?cluster=devnet",
        result.pool_address
    );
    println!(
        "Initialisation Cost: {} lamports",
        result.initialization_cost
    );
}
