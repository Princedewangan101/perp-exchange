use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppState, middlewares::auth::AuthenticatedUser,
    query::fetch_transactions::{fetch_transactions_from_db, FetchTransactionsRequest},
};

pub async fn fetch_transactions(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let response = fetch_transactions_from_db(&state.db, FetchTransactionsRequest { user_id: user.0.clone() }).await;

    if !response.success {
        return (StatusCode::CONFLICT, Json(response));
    }

    // println!("\n>[INFO] fetch_transactions , success");
    (StatusCode::OK, Json(response))
}
