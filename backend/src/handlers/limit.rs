use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::limit_order::{LimitOrderRequest, LimitOrderResponse, limit_order};

#[derive(Deserialize)]
pub struct OrderEvent {
    pub symbol: String,
    pub quantity: f64,
    pub side: u32,
    pub price: f64,
    pub order_type: String,
    pub leverage: u32,
}

#[derive(Deserialize)]
pub struct OrderEdge {
    pub tp: Option<f64>,
    pub sl: Option<f64>,
}

#[derive(Deserialize)]
pub struct MarketRequest {
    #[serde(flatten)]
    pub order: OrderEvent,

    #[serde(flatten)]
    pub edge: OrderEdge,
}

#[derive(Serialize, Debug)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
    pub order_id: Option<String>,
}

#[derive(Serialize)]
struct OrderEventPayload {
    order_id: String,
    user_id: String,
    symbol: String,
    quantity: f64,
    side: u32,
    order_type: String,
    leverage: u32,
    price: f64,
}

pub async fn limit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    // INPUT CHECK
    if req.order.symbol.is_empty()
        || req.order.quantity <= 0.0
        || req.order.side > 1
        || req.order.price <= 0.0
        || req.order.order_type.is_empty()
        || req.order.leverage == 0
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(MarketResponse {
                success: false,
                message: "missing required field".to_string(),
                order_id: None,
            }),
        );
    }

    println!(
        "\n\n\n> [LIMIT_ORDER] User: {}, Symbol: {}, Qty: {}, Side: {}, Type: {}, Status: pending, Leverage: {}, TP: {}, SL: {}, Price: {}",
        user.0,
        req.order.symbol,
        req.order.quantity,
        req.order.side,
        req.order.order_type,
        req.order.leverage,
        req.edge.tp.unwrap_or(0.0),
        req.edge.sl.unwrap_or(0.0),
        req.order.price
    );

    // QUERY CALL
    let response = limit_order(
        &state.db,
        LimitOrderRequest {
            user_id: user.0.clone(),
            symbol: req.order.symbol.clone(),
            quantity: req.order.quantity,
            side: req.order.side,
            order_type: req.order.order_type.clone(),
            status: "pending".to_string(),
            leverage: req.order.leverage,
            tp: req.edge.tp.unwrap_or(0.0),
            sl: req.edge.sl.unwrap_or(0.0),
            open: req.order.price,
        },
    )
    .await;

    // RESPONSE MATCHING
    let order_id = match response {
        LimitOrderResponse {
            success: true,
            order_id: Some(id),
        } => id,
        _ => {
            return (
                StatusCode::CONFLICT,
                Json(MarketResponse {
                    success: false,
                    message: "failed to process order".to_string(),
                    order_id: None,
                }),
            );
        }
    };


    let event_payload = OrderEventPayload {
        order_id: order_id.clone(),
        user_id: user.0.clone(),
        symbol: req.order.symbol.clone(),
        quantity: req.order.quantity,
        side: req.order.side,
        order_type: req.order.order_type.clone(),
        leverage: req.order.leverage,
        price: req.order.price,
    };
    
    // ENQUEUE
    if let Ok(bytes) = serde_json::to_vec(&event_payload) {
        if let Err(e) = state.nats.publish("order.limit", bytes.into()).await {
            eprintln!("\n[ERROR] Failed to publish event to NATS: {:?}", e);
        }
    } else {
        eprintln!("\n[ERROR] Failed to serialize order evemt payload")
    }

    println!("\n> [LIMIT_ORDER_RESPONSE]: success:true, message:order in pending, order_id:{}",order_id );
    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: true,
            message: "order in pending".to_string(),
            order_id: Some(order_id),
        }),
    );
}
