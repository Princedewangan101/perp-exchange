use dotenvy::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use tokio_postgres::{Client, Error};

pub async fn connect_postgres() -> Result<Client, Error> {
    // 1. Load environment variables
    dotenv().ok();
    let conn_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Set up the TLS connector required by Neon
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    if let Ok(ca_file) = env::var("SSL_CERT_FILE") {
        builder.set_ca_file(ca_file).unwrap();
    }
    let connector = MakeTlsConnector::new(builder.build());

    // 3. Establish connection
    let (client, connection) = tokio_postgres::connect(&conn_string, connector).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS users (
             userId SERIAL PRIMARY KEY,
             email VARCHAR(100) NOT NULL UNIQUE,
             password VARCHAR(100) NOT NULL,
             balance NUMERIC(10, 2) NOT NULL DEFAULT 0.00,
             created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
             updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
         );

           CREATE TABLE IF NOT EXISTS orders (
              orderId UUID PRIMARY KEY,
             userId INT NOT NULL REFERENCES users(userId) ON DELETE CASCADE,
             symbol VARCHAR(10) NOT NULL,
             quantity NUMERIC(4,2) NOT NULL,
             side SMALLINT NOT NULL CHECK (side IN (0, 1)),
             type VARCHAR(6) NOT NULL DEFAULT 'market',
             status VARCHAR(9) NOT NULL,
             leverage SMALLINT,
             tp NUMERIC(8,2),
             sl NUMERIC(8,2),
             open NUMERIC(8,2) NOT NULL,
             close NUMERIC(8,2),
             closeType VARCHAR(6),
             created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
             updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
         );

          CREATE TABLE IF NOT EXISTS transactions (
             transactionId SERIAL PRIMARY KEY,
             userId INT NOT NULL REFERENCES users(userId) ON DELETE CASCADE,
              orderId UUID REFERENCES orders(orderId) ON DELETE CASCADE,
             balance NUMERIC(10, 2) NOT NULL DEFAULT 0.00,
             type VARCHAR(8) NOT NULL,
             created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
         );

          CREATE INDEX IF NOT EXISTS idx_transactions_user_id
          ON transactions(userId);

          ALTER TABLE orders
          ADD COLUMN IF NOT EXISTS pnl NUMERIC(10 ,2);",
        )
        .await?;

    Ok(client)
}
