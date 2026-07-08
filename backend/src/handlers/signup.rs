use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    DbState,
    query::query::{UserStatusResponse, is_user_exist},
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    // pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub success: bool,
}

pub async fn signup(
    State(pg_client): State<DbState>,
    Json(req): Json<SignupRequest>,
) -> impl IntoResponse {
    let response: UserStatusResponse = is_user_exist(&pg_client, &req.email).await;

    let response_body: SignupResponse;

    if !response.is_user_exist {

        // hash password
        // create user
        // sign jwt
        // set jwt in cookie
        // send response

        response_body = SignupResponse {
            success: true,
            message: format!(       // ??? difference between format! and println!
                "User {}: registered successfully!",
                response.email.as_deref().unwrap_or("No email found")
            ),
        };
        (StatusCode::CREATED, Json(response_body))
    } else {
        response_body = SignupResponse {
            success: true,
            message: format!("User {}: already exist!", req.email),
        };
        (StatusCode::CREATED, Json(response_body))
    }
}
