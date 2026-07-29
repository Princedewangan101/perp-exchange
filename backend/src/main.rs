use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Method};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tokio_postgres::Client;
use tower_http::cors::CorsLayer;
mod config;
mod handlers;
mod middlewares;
mod proto;
mod query;
use config::db::postgres::connect_postgres;
use config::nats::connect_nats;
use config::redis::connect_redis;
use handlers::balance::get_balance;
use handlers::close::close;
use handlers::deposit::deposit;
use handlers::limit::limit;
use handlers::market::market;
use handlers::modify::modify;
use handlers::orderbook::get_orderbook_data;
use handlers::orders::fetch_orders;
use handlers::signin::signin;
use handlers::signup::signup;
use handlers::transactions::fetch_transactions;
use handlers::withdraw::withdraw;
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
    { "price": 61990.0, "quantity": 0.8 },
    { "price": 61980.0, "quantity": 2.1 },
    { "price": 61970.0, "quantity": 3.4 },
    { "price": 61960.0, "quantity": 1.2 },
    { "price": 61950.0, "quantity": 4.6 },
    { "price": 61940.0, "quantity": 0.7 },
    { "price": 61930.0, "quantity": 2.8 },
    { "price": 61920.0, "quantity": 5.3 },
    { "price": 61910.0, "quantity": 1.9 },
    { "price": 61900.0, "quantity": 3.1 },
    { "price": 61890.0, "quantity": 0.9 },
    { "price": 61880.0, "quantity": 6.2 },
    { "price": 61870.0, "quantity": 2.5 },
    { "price": 61860.0, "quantity": 1.8 },
    { "price": 61850.0, "quantity": 4.4 },
    { "price": 61840.0, "quantity": 0.6 },
    { "price": 61830.0, "quantity": 3.7 },
    { "price": 61820.0, "quantity": 2.2 },
    { "price": 61810.0, "quantity": 1.1 },
    { "price": 61800.0, "quantity": 5.8 },
    { "price": 61790.0, "quantity": 0.4 },
    { "price": 61780.0, "quantity": 2.6 },
    { "price": 61770.0, "quantity": 4.9 },
    { "price": 61760.0, "quantity": 1.7 },
    { "price": 61750.0, "quantity": 3.0 },
    { "price": 61740.0, "quantity": 2.4 },
    { "price": 61730.0, "quantity": 6.5 },
    { "price": 61720.0, "quantity": 0.5 },
    { "price": 61710.0, "quantity": 1.3 }
        ],
        "asks": [
            { "price": 62010.0, "quantity": 1.1 },
    { "price": 62020.0, "quantity": 2.7 },
    { "price": 62030.0, "quantity": 0.9 },
    { "price": 62040.0, "quantity": 3.8 },
    { "price": 62050.0, "quantity": 1.6 },
    { "price": 62060.0, "quantity": 4.2 },
    { "price": 62070.0, "quantity": 0.5 },
    { "price": 62080.0, "quantity": 2.9 },
    { "price": 62090.0, "quantity": 5.1 },
    { "price": 62100.0, "quantity": 1.4 },
    { "price": 62110.0, "quantity": 3.6 },
    { "price": 62120.0, "quantity": 0.8 },
    { "price": 62130.0, "quantity": 6.4 },
    { "price": 62140.0, "quantity": 2.0 },
    { "price": 62150.0, "quantity": 1.7 },
    { "price": 62160.0, "quantity": 4.8 },
    { "price": 62170.0, "quantity": 0.6 },
    { "price": 62180.0, "quantity": 3.2 },
    { "price": 62190.0, "quantity": 2.3 },
    { "price": 62200.0, "quantity": 1.9 },
    { "price": 62210.0, "quantity": 5.6 },
    { "price": 62220.0, "quantity": 0.7 },
    { "price": 62230.0, "quantity": 2.8 },
    { "price": 62240.0, "quantity": 4.5 },
    { "price": 62250.0, "quantity": 1.2 },
    { "price": 62260.0, "quantity": 3.3 },
    { "price": 62270.0, "quantity": 2.1 },
    { "price": 62280.0, "quantity": 6.0 },
    { "price": 62290.0, "quantity": 0.4 },
    { "price": 62300.0, "quantity": 1.5 }
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
        .route("/api/balance", get(get_balance))
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
