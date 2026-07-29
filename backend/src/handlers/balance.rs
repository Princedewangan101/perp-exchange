use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::{
    AppState, middlewares::auth::AuthenticatedUser,
    query::fetch_balance::{fetch_balance, FetchBalanceRequest},
};

#[derive(Serialize)]
pub struct BalanceResponse {
    success: bool,
    balance: Option<f64>,
    message: String,
}

pub async fn get_balance(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    // println!("\n>[BALANCE] fetching balance for user_id: {}", user.0);

    let response = fetch_balance(&state.db, FetchBalanceRequest { user_id: user.0 }).await;

    if !response.success {
        // println!("\n>[BALANCE] user not found");
        return (
            StatusCode::NOT_FOUND,
            Json(BalanceResponse {
                success: false,
                balance: None,
                message: "user not found".to_string(),
            }),
        );
    }

    // println!("\n>[BALANCE] balance fetched: {:?}", response.balance);
    (
        StatusCode::OK,
        Json(BalanceResponse {
            success: true,
            balance: response.balance,
            message: "balance fetched successfully".to_string(),
        }),
    )
}
