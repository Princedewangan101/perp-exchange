use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, handlers::signup::Claims, query::find_user::{find_user, FindUserRequest},
};

#[derive(Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SigninResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<String>,
    pub token: Option<String>,
}

pub async fn signin(
    State(state): State<AppState>,
    Json(req): Json<SigninRequest>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if req.email.trim().is_empty() || req.password.trim().is_empty() {
        let err_body = SigninResponse {
            success: false,
            message: "missing required field".to_string(),
            user_id: None,
            token: None,
        };
        return (StatusCode::BAD_REQUEST, headers, Json(err_body));
    }

    let response_body;

    let response = find_user(&state.db, FindUserRequest { email: req.email.clone() }).await;
    println!("\n> response: {:?}", response);
    if response.user_id.is_none() {
        response_body = SigninResponse {
            success: false,
            message: "user not exist".to_string(),
            user_id: None,
            token: None,
        };
        return (StatusCode::NOT_FOUND, headers, Json(response_body));
    }

    let user_id = response.user_id.clone();
    let my_claims = Claims {
        user_id: response.user_id.expect("user_id not found"),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(60 * 24)).timestamp() as usize,
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());

    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(token) => {
            println!("\n> token: {}", token);
            token
        }
        Err(_) => {
            let err_body = SigninResponse {
                success: false,
                message: "failed to generate session token".to_string(),
                user_id: None,
                token: None,
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, headers, Json(err_body));
        }
    };

    let cookie = Cookie::build(("token", token.clone()))
        .http_only(false)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .to_string();

    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    response_body = SigninResponse {
        success: true,
        message: "signin successful".to_string(),
        user_id: user_id.clone(),
        token: Some(token.clone()),
    };
    return (StatusCode::OK, headers, Json(response_body));
}
