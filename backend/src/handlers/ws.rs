use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc};

use crate::AppState;
use crate::middlewares::auth::Claims;

/// Query params passed by the browser when connecting: ws://host/ws?token=xxx
#[derive(Deserialize)]
pub struct WsQuery {
    pub token: Option<String>,
}

/// Shape of the `live_price` event from the engine
#[derive(Serialize, Deserialize)]
pub struct LivePrice {
    pub symbol: String,
    pub price: f64,
    pub time: i64,
}

/// Shape of the `order.filled` event from the engine
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderFilled {
    pub buy_order_id: Option<String>,
    pub sell_order_id: Option<String>,
    pub buy_user_id: Option<String>,
    pub sell_user_id: Option<String>,
    pub quantity: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBookData {
    pub symbol: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

/// Wraps event types and adds `event_type` discriminator during serialization
#[derive(Serialize)]
#[serde(tag = "event_type")]
pub enum WsEvent {
    LivePrice(LivePrice),
    OrderFilled(OrderFilled),
    OrderBook(OrderBookData),
}

/// One connected browser session
struct Session {
    tx: mpsc::UnboundedSender<String>,
    user_id: Option<String>,
}

/// Central registry of all active WebSocket sessions
///
/// Two maps:
///   `sessions` — conn_id → Session  (all connections)
///   `by_user`  — user_id → Vec<conn_id>  (fast look-up for unicast)
pub struct WsManager {
    sessions: RwLock<HashMap<u64, Session>>,
    by_user: RwLock<HashMap<String, Vec<u64>>>,
    next_id: AtomicU64,
    pub last_orderbook: RwLock<Option<String>>,
}

impl WsManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: RwLock::new(HashMap::new()),
            by_user: RwLock::new(HashMap::new()),
            next_id: AtomicU64::new(1),
            last_orderbook: RwLock::new(None),
        })
    }

    /// Register a new WebSocket session and return its unique ID
    pub async fn register(&self, tx: mpsc::UnboundedSender<String>, user_id: Option<String>) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        println!("\n> [WS_REGISTER]: conn_id={} user_id={:?} total_sessions={}", id, user_id, self.sessions.read().await.len() + 1);
        self.sessions.write().await.insert(id, Session {
            tx,
            user_id: user_id.clone(),
        });
        if let Some(uid) = user_id {
            self.by_user.write().await.entry(uid).or_default().push(id);
        }
        id
    }

    /// Remove a session on disconnect; cleans up both maps
    pub async fn unregister(&self, id: u64) {
        if let Some(session) = self.sessions.write().await.remove(&id) {
            if let Some(uid) = session.user_id {
                let mut by_user = self.by_user.write().await;
                if let Some(ids) = by_user.get_mut(&uid) {
                    ids.retain(|&i| i != id);
                    if ids.is_empty() {
                        by_user.remove(&uid);
                    }
                }
            }
        }
    }

    /// Send a message to every connected client
    pub async fn broadcast(&self, msg: &str) {
        let sessions = self.sessions.read().await;
        // println!("\n> [WS_BROADCAST]: sending to {} client(s)", sessions.len());
        for session in sessions.values() {
            let _ = session.tx.send(msg.to_string());
        }
    }

    /// Send a message only to sessions belonging to a specific user
    pub async fn unicast(&self, user_id: &str, msg: &str) {
        let by_user = self.by_user.read().await;
        if let Some(ids) = by_user.get(user_id) {
            let sessions = self.sessions.read().await;
            for id in ids {
                if let Some(session) = sessions.get(id) {
                    let _ = session.tx.send(msg.to_string());
                }
            }
        }
    }
}

/// Spawn global NATS subscribers that forward events to WebSocket clients
///
/// Called once at startup. Two background tasks:
/// - `live_price` → broadcast to every connected client
/// - `order.filled` → unicast to the buyer and seller
pub fn spawn_nats_subscribers(state: &AppState) {
    let nats = state.nats.clone();
    let wm = state.ws_manager.clone();
    tokio::spawn(async move {
        let mut sub = match nats.subscribe("live_price").await {
            Ok(s) => s,
            Err(_) => return,
        };
        while let Some(msg) = sub.next().await {
            if let Ok(price) = serde_json::from_slice::<LivePrice>(&msg.payload) {
                // println!("\n> [WS_LIVE_PRICE]: received from NATS symbol={} price={}", price.symbol, price.price);
                if let Ok(json) = serde_json::to_string(&WsEvent::LivePrice(price)) {
                    wm.broadcast(&json).await;
                }
            }
        }
    });

    {
        let nats = state.nats.clone();
        let wm = state.ws_manager.clone();
        tokio::spawn(async move {
            let mut sub = match nats.subscribe("order.filled").await {
                Ok(s) => s,
                Err(_) => return,
            };
            while let Some(msg) = sub.next().await {
                if let Ok(fill) = serde_json::from_slice::<OrderFilled>(&msg.payload) {
                    if let Ok(json) = serde_json::to_string(&WsEvent::OrderFilled(fill.clone())) {
                        if let Some(uid) = &fill.buy_user_id {
                            wm.unicast(uid, &json).await;
                        }
                        if let Some(uid) = &fill.sell_user_id {
                            wm.unicast(uid, &json).await;
                        }
                    }
                }
            }
        });
    }

    {
        let nats = state.nats.clone();
        let wm = state.ws_manager.clone();
        let redis = state.redis.clone();
        tokio::spawn(async move {
            let mut sub = match nats.subscribe("orderbook.snapshot").await {
                Ok(s) => s,
                Err(_) => return,
            };
            while let Some(msg) = sub.next().await {
                if let Ok(ob) = serde_json::from_slice::<OrderBookData>(&msg.payload) {
                    if let Ok(json) = serde_json::to_string(&WsEvent::OrderBook(ob)) {
                        let mut conn = redis.as_ref().clone();
                        let _ = redis::cmd("SET")
                            .arg("orderbook.snapshot")
                            .arg(&json)
                            .query_async::<()>(&mut conn)
                            .await;
                        *wm.last_orderbook.write().await = Some(json.clone());
                        wm.broadcast(&json).await;
                    }
                }
            }
        });
    }
}

/// Handle the HTTP → WebSocket upgrade handshake
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    // Validate JWT if token was provided (optional auth)
    let user_id = match &query.token {
        Some(token) => {
            println!("\n> [WS_HANDLER]: connection attempt with token={}", token);
            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(data) => {
                    println!("\n> [WS_HANDLER]: token valid, user_id={}", data.claims.user_id);
                    Some(data.claims.user_id)
                }
                Err(e) => {
                    println!("\n> [WS_HANDLER]: token invalid: {:?}", e);
                    return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
                }
            }
        }
        None => {
            println!("\n> [WS_HANDLER]: no token provided, connecting anonymously");
            None
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

/// Per-connection loop: reads from the session's channel and forwards to WebSocket,
/// while also handling ping/pong/close from the browser
async fn handle_socket(socket: WebSocket, state: AppState, user_id: Option<String>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<String>();

    // Register this connection so global NATS tasks can reach it
    let conn_id = state.ws_manager.register(event_tx.clone(), user_id).await;

    // Send the latest orderbook snapshot immediately on connect
    if let Some(snapshot) = state.ws_manager.last_orderbook.read().await.clone() {
        let _ = event_tx.send(snapshot);
    }

    loop {
        tokio::select! {
            // Incoming message from a global NATS subscriber (via WsManager)
            Some(msg) = event_rx.recv() => {
                if ws_tx.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            // Incoming message from the browser
            Some(msg) = ws_rx.next() => {
                match msg {
                    Ok(Message::Ping(data)) => {
                        if ws_tx.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        }
    }

    // Clean up on disconnect
    state.ws_manager.unregister(conn_id).await;
}
