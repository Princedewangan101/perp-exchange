use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Method};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tokio_postgres::Client;
use tower_http::cors::{CorsLayer};
mod config;
mod handlers;
mod middlewares;
mod proto;
mod query;
use config::db::postgres::connect_postgres;
use config::nats::connect_nats;
use config::redis::connect_redis;
use handlers::close::close;
use handlers::deposit::deposit;
use handlers::limit::limit;
use handlers::market::market;
use handlers::modify::modify;
use handlers::orders::fetch_orders;
use handlers::signin::signin;
use handlers::signup::signup;
use handlers::transactions::fetch_transactions;
use handlers::withdraw::withdraw;
use handlers::orderbook::get_orderbook_data;
use handlers::ws::{WsManager, spawn_nats_subscribers, ws_handler};
use middlewares::alt_auth_mw::alt_auth;

pub type DbState = Arc<Client>;
pub type RedisState = Arc<ConnectionManager>;

/// Shared application state injected into every request handler and WebSocket session
#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub redis: RedisState,
    pub nats: async_nats::Client,
    pub ws_manager: Arc<WsManager>,
}

/// Seed 3 bid and 3 ask dummy orders into Redis under key `"orderbook.snapshot"`.
///
/// Called once at startup so that `GET /api/orderbook` always returns data
/// even when the matching engine is not running. The format matches the
/// `WsEvent::OrderBook` shape consumed by the frontend.
async fn seed_dummy_orderbook(redis: &ConnectionManager) {
    let dummy = serde_json::json!({
        "event_type": "OrderBook",
        "symbol": "BTC-PERP",
        "bids": [
            { "price": 62000.0, "quantity": 1.5 },
            { "price": 61850.0, "quantity": 2.3 },
            { "price": 61500.0, "quantity": 0.8 },
        ],
        "asks": [
            { "price": 63500.0, "quantity": 1.2 },
            { "price": 63800.0, "quantity": 0.9 },
            { "price": 64200.0, "quantity": 2.1 },
        ],
    });
    if let Ok(json) = serde_json::to_string(&dummy) {
        let mut conn = redis.clone();
        let _ = redis::cmd("SET")
            .arg("orderbook.snapshot")
            .arg(&json)
            .query_async::<()>(&mut conn)
            .await;
        println!("\n>> seeded dummy orderbook into Redis (3 bids + 3 asks)");
    }
}

#[tokio::main]
async fn main() {
    println!("\n>> run redis and nats");

    // ── Connect to external services ──────────────────────────────────
    let pg_client = connect_postgres()
        .await
        .expect("[CRITICAL]  Failed to connect to the PostgreSQL database server");
    let redis_cm = connect_redis().await.unwrap();
    let nats_cm = connect_nats().await.unwrap();

    // ── Seed dummy orderbook into Redis for testing ──────────────────
    seed_dummy_orderbook(&redis_cm).await;

    // ── WebSocket session manager ─────────────────────────────────────
    let ws_manager = WsManager::new();
    let state = AppState {
        db: Arc::new(pg_client),
        redis: Arc::new(redis_cm),
        nats: nats_cm,
        ws_manager: ws_manager.clone(),
    };

    // ── Global NATS → WebSocket broadcast tasks ──────────────────────
    spawn_nats_subscribers(&state);

    // ── HTTP routes ──────────────────────────────────────────────────

    // Protected routes (require JWT via Authorization header)
    let protected = Router::new()
        .route("/api/deposit", post(deposit))
        .route("/api/withdraw", post(withdraw))
        .route("/api/market", post(market))
        .route("/api/limit", post(limit))
        .route("/api/modify", post(modify))
        .route("/api/close", post(close))
        .route("/api/orders", get(fetch_orders))
        .route("/api/transactions", post(fetch_transactions))
        .layer(middleware::from_fn(alt_auth))
        .with_state(state.clone());

    // CORS configuration for the Next.js frontend at localhost:3000
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .expose_headers([SET_COOKIE])
        .allow_credentials(true);

    // Public routes + WebSocket + protected routes
    let app: Router = Router::new()
        .route("/api/signup", post(signup))
        .route("/api/signin", post(signin))
        .route("/ws", get(ws_handler))
        .route("/api/orderbook", get(get_orderbook_data))
        .merge(protected)
        .layer(cors)
        .with_state(state);

    // ── Start server ─────────────────────────────────────────────────
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    eprintln!("\n>> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
