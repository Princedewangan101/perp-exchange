use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub success: bool,
}

pub async fn signup(Json(req):Json<SignupRequest>) -> impl IntoResponse {


    
    let response_body = SignupResponse {
        success: true,
        message: format!("User {}: {} registered successfully!", req.email, req.password),
    };

    (StatusCode::CREATED, Json(response_body))
}

