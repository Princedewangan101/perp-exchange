use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    query::query::{UserStatusResponse, create_user, is_user_exist},
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize, // what is usize ???
}

pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if req.email.trim().is_empty() || req.password.trim().is_empty() {
        let err_body = SignupResponse {
            success: false,
            message: "missing required data".to_string(),
            user_id: None,
        };
        return (StatusCode::BAD_REQUEST, headers, Json(err_body));
    }

    let response: UserStatusResponse = is_user_exist(&state.db, &req.email).await;

    let response_body: SignupResponse;

    if !response.is_user_exist {
        // hash password
        let hashed_password = match hash(&req.password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => {
                let err_body = SignupResponse {
                    success: false,
                    message: "internal server error during password security encryption"
                        .to_string(),
                    user_id: None,
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, headers, Json(err_body));
            }
        };

        // create user // there is a create_user fn in query.rs use that
        let create_user_response = create_user(&state.db, &req.email, &hashed_password).await;

        if !create_user_response.success {
            response_body = SignupResponse {
                success: create_user_response.success,
                message: format!("failed to register user"),
                user_id: None,
            };
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                Json(response_body),
            );
        }

        let my_claims = Claims {
            user_id: create_user_response.id.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(1)).timestamp() as usize,
        };
        println!("\n> my_claims = {:?}", my_claims);
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());

        // sign jwt,  payload will be {userId:userId}
        let token = match encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        ) {
            Ok(token) => token,
            Err(_) => {
                let err_body = SignupResponse {
                    success: false,
                    message: "failed to generate session token".to_string(),
                    user_id: None,
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, headers, Json(err_body));
            }
        };

        // set jwt in cookie
        let cookie = Cookie::build(("token", token))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .to_string();

        headers.insert(SET_COOKIE, cookie.parse().unwrap());

        // send response
        response_body = SignupResponse {
            success: true,
            message: format!(
                // ??? difference between format! and println!
                "User registered successfully!",
            ),
            user_id: Some(create_user_response.id),
        };
        return (StatusCode::CREATED, headers, Json(response_body));
    } else {
        response_body = SignupResponse {
            success: true,
            message: format!("User {}: already exist!", req.email),
            user_id: None,
        };
        return (StatusCode::CREATED, headers, Json(response_body));
    }
}
