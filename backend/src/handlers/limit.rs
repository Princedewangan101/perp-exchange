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
    pub remaining_quantity: Option<f64>,
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
    tp: Option<f64>,
    sl: Option<f64>,
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
                remaining_quantity: None,
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
                    remaining_quantity: None,
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
        tp: req.edge.tp.unwrap_or(0.0),
        sl: req.edge.sl.unwrap_or(0.0),
    };
    
    // ENQUEUE
    if let Ok(bytes) = serde_json::to_vec(&event_payload) {
        match state.nats.request("order.limit", bytes.into()).await {
            Ok(reply_msg) => {
                match serde_json::from_slice::<serde_json::Value>(&reply_msg.payload) {
                    Ok(reply) => {
                        let success = reply.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                        let message = reply.get("message").and_then(|v| v.as_str()).unwrap_or("unknown");
                        let remaining = reply.get("remaining_quantity").and_then(|v| v.as_f64());
                        println!("\n> [LIMIT_ORDER_RESPONSE]: success:{success}, message:{message}, order_id:{order_id}, remaining_quantity:{remaining:?}");
                        return (
                            StatusCode::OK,
                            Json(MarketResponse {
                                success,
                                message: format!("{}, remaining: {:?}", message, remaining),
                                order_id: Some(order_id),
                                remaining_quantity: remaining,
                            }),
                        );
                    }
                    Err(_) => {
                        eprintln!("\n[ERROR] Failed to decode engine reply for order.limit");
                        return (
                            StatusCode::OK,
                            Json(MarketResponse {
                                success: true,
                                message: "order placed, but failed to decode engine reply".to_string(),
                                order_id: Some(order_id),
                                remaining_quantity: None,
                            }),
                        );
                    }
                }
            }
            Err(_) => {
                eprintln!("\n[ERROR] NATS request timeout for order.limit");
                return (
                    StatusCode::GATEWAY_TIMEOUT,
                    Json(MarketResponse {
                        success: false,
                        message: "matching engine timeout".to_string(),
                        order_id: None,
                        remaining_quantity: None,
                    }),
                );
            }
        }
    } else {
        eprintln!("\n[ERROR] Failed to serialize order event payload");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "serialization error".to_string(),
                order_id: None,
                remaining_quantity: None,
            }),
        );
    }
}
