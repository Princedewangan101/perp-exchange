use dotenvy::dotenv;
use redis::aio::ConnectionManager;
use redis::RedisError;
use std::env;

pub async fn connect_redis() -> Result<ConnectionManager, RedisError> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let client = redis::Client::open(redis_url.as_str())?;
    let connection = ConnectionManager::new(client).await?;

    Ok(connection)
}
