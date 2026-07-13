use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::query::limit_order;

#[derive(Deserialize)]
pub struct OrderEvent {
    pub symbol: String,
    pub quantity: u32,
    pub side: u32,
    pub price: u64,
    pub order_type: String,
    pub leverage: u32,
}

#[derive(Deserialize)]
pub struct OrderEdge {
    pub tp: Option<u64>,
    pub sl: Option<u64>,
}

#[derive(Deserialize)]
pub struct MarketRequest {
    #[serde(flatten)]
    pub order: OrderEvent,

    #[serde(flatten)]
    pub edge: OrderEdge,
}

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
}

pub async fn limit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    if req.order.symbol.is_empty()
        || req.order.quantity == 0
        || req.order.side > 1
        || req.order.price > 0
        || req.order.order_type.is_empty()
        || req.order.leverage == 0
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(MarketResponse {
                success: false,
                message: "missing required field".to_string(),
            }),
        );
    }

    let proto_req = proto::LimitOrderRequest {
        user_id: user.0.clone(),
        symbol: req.order.symbol.clone(),
        quantity: req.order.quantity as u32,
        side: req.order.side as u32,
        price: req.order.price as u64,
        order_type: req.order.order_type.clone(),
        tp: req.edge.tp,
        sl: req.edge.sl,
    };

    let mut req_buffer = Vec::new();

    if proto_req.encode(&mut req_buffer).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "encode error".to_string(),
            }),
        );
    }

    state
        .nats
        .publish("orders.limit".to_string(), req_buffer.into())
        .await
        .expect("Failed to publish event");

    // state.nats.request("LIMIT_ORDER", req_buffer.into()).await;

    let response = limit_order(
        &state.db,
        &user.0,
        &req.order.symbol,
        &req.order.quantity,
        &req.order.side,
        &req.order.order_type,
        "pending".to_string(),
        &req.order.leverage,
        &req.edge.tp.unwrap_or(0),
        &req.edge.sl.unwrap_or(0),
        &req.order.price,
    )
    .await;

    if !response {
        return (
            StatusCode::CONFLICT,
            Json(MarketResponse {
                success: false,
                message: "failed to process order".to_string(),
            }),
        );
    }

    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: true,
            message: "order in pending".to_string(),
        }),
    );
}
