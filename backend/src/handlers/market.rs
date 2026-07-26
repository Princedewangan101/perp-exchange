use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;

/// Incoming order details from the client.
#[derive(Deserialize)]
pub struct OrderEvent {
    /// Trading pair symbol (e.g. "BTCUSD").
    pub symbol: String,
    /// Order quantity (must be > 0).
    pub quantity: f64,
    /// Order side: 0 = buy, 1 = sell.
    pub side: u8,
    /// Order type identifier (e.g. "market", "limit").
    pub order_type: String,
}

/// Take-profit and stop-loss levels attached to an order.
#[derive(Deserialize)]
pub struct OrderEdge {
    /// Take-profit price (negative values treated as unset).
    pub tp: f64,
    /// Stop-loss price (negative values treated as unset).
    pub sl: f64,
}

/// Top-level request body accepted by the market endpoint.
///
/// Both `OrderEvent` and `OrderEdge` fields are flattened into a single JSON
/// payload so callers send all fields at the top level.
#[derive(Deserialize)]
pub struct MarketRequest {
    #[serde(flatten)]
    pub order: OrderEvent,

    #[serde(flatten)]
    pub edge: OrderEdge,
}

/// Response returned by the market endpoint.
#[derive(Serialize)]
pub struct MarketResponse {
    /// Whether the order was accepted by the matching engine.
    pub success: bool,
    /// Human-readable status or error description.
    pub message: String,
}

/// Place a market order.
///
/// Validates the request, encodes it as a protobuf `OrderRequest`, and forwards
/// it over NATS to the matching engine. Returns the engine's response or an
/// appropriate HTTP error on failure.
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
            }),
        );
    }

    let proto_req = proto::OrderRequest {
        user_id: user.0,
        symbol: req.order.symbol,
        quantity: req.order.quantity,
        side: req.order.side as u32,
        order_type: req.order.order_type,
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

    match state.nats.request("MARKET_ORDER", req_buffer.into()).await {
        Ok(reply_message) => match proto::OrderResponse::decode(reply_message.payload) {
            Ok(proto_res) => {
                return (
                    StatusCode::OK,
                    Json(MarketResponse {
                        success: true,
                        message: format!("{} , Qty: {}", proto_res.message, proto_res.quantity),
                    }),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MarketResponse {
                        success: false,
                        message: "decode error".to_string(),
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
                }),
            );
        }
    }
}
