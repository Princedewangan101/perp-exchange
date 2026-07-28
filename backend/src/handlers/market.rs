use uuid::Uuid;
use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::market_order::{market_order, MarketOrderRequest};

#[derive(Deserialize)]
pub struct OrderEvent {
    pub symbol: String,
    pub quantity: f64,
    pub side: u8,
    pub order_type: String,
}

#[derive(Deserialize)]
pub struct OrderEdge {
    pub tp: f64,
    pub sl: f64,
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

pub async fn market(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    if req.order.symbol.is_empty()
        || req.order.quantity <= 0.0
        || req.order.side > 1
        || req.order.order_type.is_empty()
        || req.edge.tp < 0.0
        || req.edge.sl < 0.0
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

    let order_id = Uuid::new_v4().to_string();

    let proto_req = proto::OrderRequest {
        user_id: user.0.clone(),
        symbol: req.order.symbol.clone(),
        quantity: req.order.quantity,
        side: req.order.side as u32,
        order_type: req.order.order_type.clone(),
        order_id: order_id.clone(),
    };

    let mut req_buffer = Vec::new();
    if proto_req.encode(&mut req_buffer).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "encode error".to_string(),
                order_id: None,
            }),
        );
    }

    let nats_result = match state.nats.request("order.market", req_buffer.into()).await {
        Ok(reply_message) => match proto::OrderResponse::decode(reply_message.payload) {
            Ok(proto_res) => proto_res,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MarketResponse {
                        success: false,
                        message: "decode error".to_string(),
                        order_id: None,
                    }),
                );
            }
        },
        Err(_) => {
            return (
                StatusCode::GATEWAY_TIMEOUT,
                Json(MarketResponse {
                    success: false,
                    message: "matching engine timeout".to_string(),
                    order_id: None,
                }),
            );
        }
    };

    let fill_price = nats_result.price;

    let db_success = market_order(
        &state.db,
        MarketOrderRequest {
            order_id: order_id.clone(),
            user_id: user.0.clone(),
            symbol: req.order.symbol.clone(),
            quantity: req.order.quantity,
            side: req.order.side as u32,
            order_type: req.order.order_type.clone(),
            status: "completed".to_string(),
            leverage: 1,
            tp: req.edge.tp,
            sl: req.edge.sl,
            open: fill_price,
        },
    )
    .await;

    if !db_success.success {
        eprintln!("\n[ERROR] Failed to persist market order {order_id} to database");
    }

    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: true,
            message: format!("{}, Qty: {}, Price: {}", nats_result.message, nats_result.quantity, nats_result.price),
            order_id: Some(order_id),
        }),
    );
}