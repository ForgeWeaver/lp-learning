mod send_sol;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok(); // Ignore if .env not found, falls back to system env

    send_sol::run().await?;
    Ok(())
}
