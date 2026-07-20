use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

pub async fn alt_auth(mut req: Request, next: Next) -> Response {
    // println!("[ALT AUTH MW] Processing incoming request to URI: {}", req.uri());

    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    // println!("[ALT AUTH MW] Authorization header: {:?}", auth_header);

    let token = match auth_header {
        Some(header) => {
            if let Some(token) = header.strip_prefix("Bearer ") {
                // println!("[ALT AUTH MW] Found Bearer token");
                token.to_string()
            } else {
                // println!("[ALT AUTH MW] Error: Authorization header missing 'Bearer ' prefix");
                return (StatusCode::UNAUTHORIZED, "missing token").into_response();
            }
        }
        None => {
            // println!("[ALT AUTH MW] Error: Authorization header missing");
            return (StatusCode::UNAUTHORIZED, "missing token").into_response();
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());

    let claims = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => {
            // println!("[ALT AUTH MW] Success: JWT validated. Claims: {:?}", data.claims);
            data.claims
        }
        Err(e) => {
            // println!("[ALT AUTH MW] Error: JWT validation failed. Reason: {:?}", e);
            return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
        }
    };

    // println!("[ALT AUTH MW] Inserting user_id '{}' into request extensions", claims.user_id);
    req.extensions_mut().insert(claims.user_id);

    // println!("[ALT AUTH MW] Passing request to next handler");
    next.run(req).await
}
