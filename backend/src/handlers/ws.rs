use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::middlewares::auth::Claims;
use crate::AppState;

#[derive(Deserialize)]
pub struct WsQuery {
    token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LivePrice {
    pub symbol: String,
    pub price: f64,
    pub time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct OrderFilled {
    pub buy_order_id: Option<String>,
    pub sell_order_id: Option<String>,
    pub buy_user_id: Option<String>,
    pub sell_user_id: Option<String>,
    pub quantity: f64,
}

#[derive(Serialize)]
#[serde(tag = "event_type")]
pub enum WsEvent {
    LivePrice(LivePrice),
    OrderFilled(OrderFilled),
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let user_id = match &query.token {
        Some(token) => {
            let jwt_secret =
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(data) => Some(data.claims.user_id),
                Err(_) => {
                    return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
                }
            }
        }
        None => None,
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: Option<String>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<WsEvent>(); // putting nats payload into this event.

    let nats = state.nats.clone();
    let tx1 = event_tx.clone();
    tokio::spawn(async move {
        let mut sub = match nats.subscribe("live_price").await {
            Ok(s) => s,
            Err(_) => return,
        };
        while let Some(msg) = sub.next().await {
            if let Ok(price) = serde_json::from_slice::<LivePrice>(&msg.payload) {
                if tx1.send(WsEvent::LivePrice(price)).is_err() {
                    break;
                }
            }
        }
    });

    if let Some(_uid) = user_id {
        let nats2 = state.nats.clone();
        let tx2 = event_tx.clone();
        tokio::spawn(async move {
            let mut sub = match nats2.subscribe("order.filled").await {
                Ok(s) => s,
                Err(_) => return,
            };
            while let Some(msg) = sub.next().await {
                if let Ok(fill) = serde_json::from_slice::<OrderFilled>(&msg.payload) {
                    if tx2.send(WsEvent::OrderFilled(fill)).is_err() {
                        break;
                    }
                }
            }
        });
    }

    loop {
        tokio::select! {
            Some(event) = event_rx.recv() => {
                if let Ok(json) = serde_json::to_string(&event) {
                    if ws_tx.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
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
}
