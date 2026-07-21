use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::modify_order::{modify_order, ModifyOrderRequest, ModifyOrderResponse};

#[derive(Deserialize)]
pub struct MarketRequest {
    pub order_id: String,
    pub symbol: String,
    pub tp: f64,
    pub sl: f64,
}

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
    pub symbol: String,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
}

pub async fn modify(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    let proto_req = proto::ModifyOrderRequest {
        user_id: user.0.clone(),
        order_id: req.order_id.clone(),
        symbol: req.symbol.clone(),
        tp: req.tp,
        sl: req.sl,
    };

    let mut req_buffer = Vec::new();

    if proto_req.encode(&mut req_buffer).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "encode error".to_string(),
                symbol: req.symbol.clone(),
                tp: None,
                sl: None,
            }),
        );
    }

    let response = modify_order(
        &state.db,
        ModifyOrderRequest {
            user_id: user.0.clone(),
            order_id: req.order_id.clone(),
            tp: req.tp,
            sl: req.sl,
        },
    )
    .await;

    match response {
        ModifyOrderResponse { success: true, tp: Some(updated_tp), sl: Some(updated_sl) } => {
            return (
                StatusCode::OK,
                Json(MarketResponse {
                    success: true,
                    message: "modified successfully".to_string(),
                    symbol: req.symbol.clone(),
                    tp: Some(updated_tp),
                    sl: Some(updated_sl),
                }),
            );
        }
        _ => {
            return (
                StatusCode::CONFLICT,
                Json(MarketResponse {
                    success: false,
                    message: "failed to modify".to_string(),
                    symbol: req.symbol.clone(),
                    tp: None,
                    sl: None,
                }),
            );
        }
    }
}
