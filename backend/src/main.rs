use axum::{Router, middleware, routing::post};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tokio_postgres::Client;

mod config;
mod handlers;
mod middlewares;
mod query;

use config::db::postgres::connect_postgres;
use config::redis::connect_redis;
use config::nats::connect_nats;
use handlers::deposit::deposit;
use handlers::withdraw::withdraw;
use handlers::signin::signin;
use handlers::signup::signup;
use middlewares::auth::auth;

pub type DbState = Arc<Client>;
pub type RedisState = Arc<ConnectionManager>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub redis: RedisState,
    pub nats: async_nats::Client,
}

#[tokio::main]
async fn main() {
    let pg_client = connect_postgres().await.unwrap();
    let redis_cm = connect_redis().await.unwrap();
    let nats_cm = connect_nats().await.unwrap();

    let state = AppState {
        db: Arc::new(pg_client),
        redis: Arc::new(redis_cm),
        nats: nats_cm
    };

    let protected = Router::new()
        .route("/api/deposit", post(deposit))
        .route("/api/withdraw", post(withdraw))
        .layer(middleware::from_fn(auth))
        .with_state(state.clone());

    let app: Router = Router::new()
        .route("/api/signup", post(signup))
        .route("/api/signin", post(signin))
        .merge(protected)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    eprintln!("\n>> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
