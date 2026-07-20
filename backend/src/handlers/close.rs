use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::query::query::{close_order, update_balance};

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

    let response = close_order(&state.db, &user.0, &req.order_id, &close_price, "manual").await;

    if !response {
        return (
            StatusCode::CONFLICT,
            Json(MarketResponse {
                success: false,
                balance: None,
                message: "failed to close order".to_string(),
            }),
        );
    }

    let balance_response = update_balance(&state.db, &req.order_id, &user.0, &close_price).await;

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
