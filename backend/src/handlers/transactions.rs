use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{AppState, middlewares::auth::AuthenticatedUser, query::query::fetch_transactions_from_db};

pub async fn fetch_transactions(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let response = fetch_transactions_from_db(&state.db, &user.0).await;

    if !response.success {
        return (
            StatusCode::CONFLICT,
            Json(response),
        );
    }

    (StatusCode::OK, Json(response))
}
