use axum::{Router, middleware, routing::post};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tokio_postgres::Client;
use tower_http::cors::{CorsLayer, Any};

mod config;
mod handlers;
mod middlewares;
mod proto;
mod query;

use config::db::postgres::connect_postgres;
use config::redis::connect_redis;
use config::nats::connect_nats;
use handlers::deposit::deposit;
use handlers::withdraw::withdraw;
use handlers::market::market;
use handlers::limit::limit;
use handlers::modify::modify;
use handlers::close::close;
use handlers::orders::fetch_orders;
use handlers::transactions::fetch_transactions;
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
    println!("\n>> run redis and nats");
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
        .route("/api/market", post(market))
        .route("/api/limit", post(limit))
        .route("/api/modify", post(modify))
        .route("/api/close", post(close))
        .route("/api/orders", post(fetch_orders))
        .route("/api/transactions", post(fetch_transactions))
        .layer(middleware::from_fn(auth))
        .with_state(state.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = Router::new()
        .route("/api/signup", post(signup))
        .route("/api/signin", post(signin))
        .merge(protected)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    eprintln!("\n>> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
