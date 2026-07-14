use crate::{AppState, middlewares::auth::AuthenticatedUser};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::proto;
use crate::query::query::modify_order;

#[derive(Deserialize)]
pub struct MarketRequest {
    pub tp: f64,
    pub sl: f64,
}

#[derive(Serialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
}

pub async fn modify(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(req): Json<MarketRequest>,
) -> impl IntoResponse {
    let proto_req = proto::ModifyOrderRequest {
        user_id: user.0.clone(),
        tp: req.tp,
        sl: req.sl,
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
        .request("orders.modify".to_string(), req_buffer.into())
        .await
    {
        Ok(reply_message) => match proto::ModifyResponse::decode(reply_message.payload) {
            Ok(proto_res) => {
                let response = modify_order(&state.db, &user.0, &req.tp, &req.sl).await;

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
