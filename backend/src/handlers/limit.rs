use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::limit_order::{limit_order, LimitOrderRequest, LimitOrderResponse};

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

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
    pub order_id: Option<String>,
}

pub async fn limit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
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

    let order_id = match response {
        LimitOrderResponse { success: true, order_id: Some(id) } => id,
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

    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: true,
            message: "order in pending".to_string(),
            order_id: Some(order_id),
        }),
    );
}
