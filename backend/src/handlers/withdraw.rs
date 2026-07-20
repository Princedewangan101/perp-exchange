use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{query::query::withdraw_balance, AppState, middlewares::auth::AuthenticatedUser};

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
        return (StatusCode::BAD_REQUEST, Json(WithdrawResponse{success:false, message:"amount cant be negative".to_string()}))
    }

    let response = withdraw_balance(&state.db, &user.0, &req.amount).await;

    if !response {
        return (StatusCode::CONFLICT, Json(WithdrawResponse{success:false, message:"failed to withdraw".to_string()}))
    }

    return (StatusCode::OK, Json(WithdrawResponse{success:true, message:"withdraw successful".to_string()}))
}
