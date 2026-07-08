use axum::{Router, routing::post};
use std::sync::Arc;
use tokio_postgres::Client;

mod config;
mod handlers;
mod query;

use handlers::signup::signup;
use handlers::signin::signin;
use config::db::postgres::connect_postgres;

pub type DbState = Arc<Client>;

#[tokio::main]
async fn main() {
    let pg_client = connect_postgres().await.unwrap();

    let shared_pg_client: DbState = Arc::new(pg_client);

    let app: Router = Router::new()
        .route("/api/signup", post(signup))
        .route("/api/signin", post(signin))
        .with_state(shared_pg_client); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("\n>> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
