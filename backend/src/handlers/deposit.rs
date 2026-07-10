use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{AppState, middlewares::auth::AuthenticatedUser, query::query::deposit_balance};

#[derive(Deserialize)]
pub struct DepositRequest {
    amount: i64,
}

#[derive(Serialize)]
pub struct DepositResponse {
    success: bool,
    message: String,
}

pub async fn deposit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<DepositRequest>,
) -> impl IntoResponse {
    if body.amount.is_negative() {
        return (StatusCode::BAD_REQUEST, Json(DepositResponse {
            success: false,
            message: "missing required field".to_string(),
        }));
    }

    let response = deposit_balance(&state.db, &user.0, &body.amount.to_string()).await;

    if !response.success {
        return (StatusCode::CONFLICT, Json(DepositResponse {
            success: false,
            message: "failed to deposit".to_string(),
        }));
    }

    return (StatusCode::OK, Json(DepositResponse {
        success: true,
        message: "deposit successful".to_string(),
    }))
}
