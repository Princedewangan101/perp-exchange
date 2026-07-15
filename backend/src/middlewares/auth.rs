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

pub struct AuthenticatedUser(pub String);

impl<S> axum::extract::FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<String>()
            .cloned()
            .map(AuthenticatedUser)
            .ok_or((StatusCode::UNAUTHORIZED, "unauthorized"))
    }
}

pub async fn auth(mut req: Request, next: Next) -> Response {
    // println!("\n[AUTH MW] Processing incoming request to URI: {}", req.uri());

    // 1. Get raw cookie header
    let cookie_header = req.headers().get("cookie").and_then(|v| v.to_str().ok());
    // println!("\n[AUTH MW] Raw cookie header found: {:?}", cookie_header);

    // 2. Extract the 'token' key from the cookie string
    let token = cookie_header.and_then(|cookies| {
        cookies.split(';').find_map(|c| {
            let c = c.trim();
            if c.starts_with("token=") {
                // println!("\n[AUTH MW] Found 'token=' cookie string");
                Some(c[6..].to_string())
            } else {
                None
            }
        })
    });

    let token = match token {
        Some(t) => t,
        None => {
            // println!("\n[AUTH MW] Error: 'token' key missing or cookie header empty");
            return (StatusCode::UNAUTHORIZED, "missing token").into_response();
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());

    // 3. Decode and validate JWT token
    let claims = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => {
            // println!("\n[AUTH MW] Success: JWT validated. Claims: {:?}", data.claims);
            data.claims
        }
        Err(e) => {
            // println!("\n[AUTH MW] Error: JWT validation failed. Reason: {:?}", e);
            return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
        }
    };

    // 4. Inject into extensions
    // println!("\n[AUTH MW] Inserting user_id '{}' into request extensions", claims.user_id);
    req.extensions_mut().insert(claims.user_id);

    // println!("\n[AUTH MW] Passing request to next handler");
    next.run(req).await
}