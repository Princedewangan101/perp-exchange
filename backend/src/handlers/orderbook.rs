use axum::Json;
use axum::extract::State;
use serde_json::Value;

use crate::AppState;

pub async fn get_orderbook_data(
    State(state): State<AppState>,
) -> Json<Value> {
    let mut conn = state.redis.as_ref().clone();
    let data: Option<String> = redis::cmd("GET")
        .arg("orderbook.snapshot")
        .query_async(&mut conn)
        .await
        .unwrap_or(None);

    match data {
        Some(raw) => {
            match serde_json::from_str::<Value>(&raw) {
                Ok(json) => Json(json),
                Err(_) => Json(Value::Null),
            }
        }
        None => Json(Value::Null),
    }
}
