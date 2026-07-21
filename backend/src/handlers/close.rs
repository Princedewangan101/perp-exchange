use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::query::close_order::{close_order, CloseOrderRequest};
use crate::query::update_balance::{update_balance, UpdateBalanceRequest};

#[derive(Deserialize)]
pub struct MarketRequest {
    pub order_id: String,
}

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub balance: Option<f64>,
    pub message: String,
}

pub async fn close(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    if req.order_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(MarketResponse {
                success: false,
                balance: None,
                message: "missing required feild".to_string(),
            }),
        );
    }

    let close_price = 999.34;

    let response = close_order(
        &state.db,
        CloseOrderRequest {
            user_id: user.0.clone(),
            order_id: req.order_id.clone(),
            close_price,
            close_type: "manual".to_string(),
        },
    )
    .await;

    if !response.success {
        return (
            StatusCode::CONFLICT,
            Json(MarketResponse {
                success: false,
                balance: None,
                message: "failed to close order".to_string(),
            }),
        );
    }

    let balance_response = update_balance(
        &state.db,
        UpdateBalanceRequest {
            order_id: req.order_id.clone(),
            user_id: user.0.clone(),
            close_price,
        },
    )
    .await;

    if !balance_response.success {
        return (
            StatusCode::CONFLICT,
            Json(MarketResponse {
                success: false,
                balance: None,
                message: "failed to close order".to_string(),
            }),
        );
    }

    return (
        StatusCode::OK,
        Json(MarketResponse {
            success: true,
            balance: balance_response.balance,
            message: "order closed".to_string(),
        }),
    );
}
