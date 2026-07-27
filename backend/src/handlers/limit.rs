use uuid::Uuid;
use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto::{LimitOrderPayload, LimitOrderResult};
use crate::query::limit_order::{LimitOrderRequest, limit_order};

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

    // GENERATE ORDER ID
    let order_id = Uuid::new_v4().to_string();

    println!(
        "\n\n\n> [LIMIT_ORDER] order_id: {order_id}, User: {}, Symbol: {}, Qty: {}, Side: {}, Type: {}, Status: pending, Leverage: {}, TP: {}, SL: {}, Price: {}",
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

    // SEND TO MATCHING ENGINE FIRST
    let event_payload = LimitOrderPayload {
        order_id: order_id.clone(),
        user_id: user.0.clone(),
        symbol: req.order.symbol.clone(),
        quantity: req.order.quantity,
        side: req.order.side,
        price: req.order.price,
        tp: req.edge.tp,
        sl: req.edge.sl,
    };

    println!("\n> [LIMIT_ORDER_NATS_SEND]: order_id:{order_id}, user_id:{}, symbol:{}, quantity:{}, side:{}, price:{}, tp:{:?}, sl:{:?}",
        user.0, req.order.symbol, req.order.quantity, req.order.side, req.order.price, req.edge.tp, req.edge.sl);

    let mut buf = Vec::new();
    if event_payload.encode(&mut buf).is_err() {
        eprintln!("\n[ERROR] Failed to encode limit order payload");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "payload encoding error".to_string(),
                order_id: None,
                remaining_quantity: None,
            }),
        );
    }

    let nats_result = match state.nats.request("order.limit", buf.into()).await {
        Ok(reply_msg) => match LimitOrderResult::decode(&reply_msg.payload[..]) {
            Ok(reply) => reply,
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
        },
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
    };

    println!("\n> [LIMIT_ORDER_RESPONSE]: success:{}, message:{}, order_id:{}, remaining_quantity:{:?}",
        nats_result.success, nats_result.message, order_id, nats_result.remaining_quantity);

    // SAVE TO DB AFTER MATCHING
    let db_success = limit_order(
        &state.db,
        LimitOrderRequest {
            order_id: order_id.clone(),
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

    if !db_success.success {
        eprintln!("\n[ERROR] Failed to persist order {order_id} to database after matching");
    }

    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: nats_result.success,
            message: format!("{}, remaining: {:?}", nats_result.message, nats_result.remaining_quantity),
            order_id: Some(order_id),
            remaining_quantity: nats_result.remaining_quantity,
        }),
    );
}
