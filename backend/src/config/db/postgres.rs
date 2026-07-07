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
            "DROP TABLE IF EXISTS todos; CREATE TABLE todos 
    (
    id SERIAL PRIMARY KEY,
    task TEXT NOT NULL
    );",
        )
        .await?;


    Ok(client)
}
