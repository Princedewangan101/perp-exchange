use async_nats::Client;
use std::env;

/// Initializes and returns a centralized NATS client connection
pub async fn connect_nats() -> Result<Client, async_nats::Error> {
    // Read the NATS URL from environment variables, fallback to Docker Desktop localhost
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    
    println!("Connecting to NATS at {}...", nats_url);
    
    // Establish the async connection
    let client = async_nats::connect(nats_url).await?;
    
    println!("Successfully connected to NATS!");
    Ok(client)
}
