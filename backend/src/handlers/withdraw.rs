use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, middlewares::auth::AuthenticatedUser,
    query::withdraw_balance::{withdraw_balance, WithdrawBalanceRequest},
};

#[derive(Deserialize)]
pub struct WithdrawRequest {
    amount: f64,
}

#[derive(Serialize)]
pub struct WithdrawResponse {
    success: bool,
    message: String,
}

pub async fn withdraw(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<WithdrawRequest>,
) -> impl IntoResponse {
    if req.amount.is_sign_negative() {
        return (
            StatusCode::BAD_REQUEST,
            Json(WithdrawResponse {
                success: false,
                message: "amount cant be negative".to_string(),
            }),
        );
    }

    let response = withdraw_balance(&state.db, WithdrawBalanceRequest { user_id: user.0.clone(), amount: req.amount }).await;

    if !response.success {
        return (
            StatusCode::CONFLICT,
            Json(WithdrawResponse {
                success: false,
                message: "failed to withdraw".to_string(),
            }),
        );
    }

    return (
        StatusCode::OK,
        Json(WithdrawResponse {
            success: true,
            message: "withdraw successful".to_string(),
        }),
    );
}
