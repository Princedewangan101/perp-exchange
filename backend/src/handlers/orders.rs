use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppState, middlewares::auth::AuthenticatedUser,
    query::fetch_orders::{fetch_orders_from_db, FetchOrdersRequest},
};

pub async fn fetch_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let response = fetch_orders_from_db(&state.db, FetchOrdersRequest { user_id: user.0.clone() }).await;

    if !response.success {
        return (StatusCode::CONFLICT, Json(response));
    }

    (StatusCode::OK, Json(response))
}
