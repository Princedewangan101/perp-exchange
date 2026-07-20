use async_nats::Client;
use std::env;

pub async fn connect_nats() -> Result<Client, async_nats::Error> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());

    let client = async_nats::connect(nats_url).await?;
    Ok(client)
}
