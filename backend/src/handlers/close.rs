use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::query::close_order;

#[derive(Deserialize)]
pub struct MarketRequest {
    pub order_id: String,
}

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
}

pub async fn close(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
if req.order_id.is_empty() {
    return (StatusCode::BAD_REQUEST, Json(MarketResponse { success: false, message: "missing required feild".to_string() }));
}

    let proto_req = proto::CloseOrderRequest {
        user_id: user.0.clone(),
        order_id: req.order_id.clone(),
    };

    let mut req_buffer = Vec::new();

    if proto_req.encode(&mut req_buffer).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MarketResponse {
                success: false,
                message: "encode error".to_string(),
            }),
        );
    }

    match state
        .nats
        .request("orders.close".to_string(), req_buffer.into())
        .await
    {
        Ok(reply_message) => match proto::CloseOrderResponse::decode(reply_message.payload) {
            Ok(proto_res) => {
                let response = close_order(&state.db, &user.0, &proto_res.close_price, "manual").await;

                if !response {
                    return (
                        StatusCode::CONFLICT,
                        Json(MarketResponse {
                            success: false,
                            message: "failed to modify".to_string(),
                        }),
                    );
                }

                return (
                    StatusCode::OK,
                    Json(MarketResponse {
                        success: true,
                        message: format!("{}", proto_res.message),
                    }),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MarketResponse {
                        success: false,
                        message: "decode error".to_string(),
                    }),
                );
            }
        },
        Err(_) => {
            return (
                StatusCode::GATEWAY_TIMEOUT,
                Json(MarketResponse {
                    success: false,
                    message: "matching engine timeout".to_string(),
                }),
            );
        }
    }
}
