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
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            balance NUMERIC(12, 0) NOT NULL DEFAULT 0.00,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        DO $$ BEGIN
            CREATE TYPE txType AS ENUM ('deposit', 'withdraw', 'profit', 'loss');
            CREATE TYPE orderType AS ENUM ('spot', 'perpf');
            CREATE TYPE orderCloseType AS ENUM ('tp', 'sl', 'manual', 'lowBalance');
            CREATE TYPE sideType AS ENUM (0, 1);
        EXCEPTION
            WHEN duplicate_object THEN NULL;
        END $$;

        CREATE TABLE IF NOT EXISTS transactions (
            transactionId SERIAL PRIMARY KEY,
            userId INT NOT NULL REFERENCES users(userId) ON DELETE CASCADE,
            balance NUMERIC(12, 0) NOT NULL DEFAULT 0.00,
            type txType NOT NULL 
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS orders (
            orderId SERIAL PRIMARY KEY,
            userId INT NOT NULL REFERENCES users(userId) ON DELETE CASCADE,
            symbol TEXT NOT NULL,
            quantity INT NOT NULL,
            side SMALLINT NOT NULL CHECK (side IN (0, 1)),
            type orderType NOT NULL DEFAULT 'spot',
            tp INT,
            sl INT,
            open INT NOT NULL,
            close INT NOT NULL,
            closeType orderCloseType NOT NULL DEFAULT 'manual',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_transactions_user_id 
        ON transactions(userId);
        ",
    )
    .await?;

    Ok(client)
}
