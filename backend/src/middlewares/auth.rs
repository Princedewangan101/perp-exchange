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
    let cookie_header = req.headers().get("cookie").and_then(|v| v.to_str().ok());

    let token = cookie_header.and_then(|cookies| {
        cookies.split(';').find_map(|c| {
            let c = c.trim();
            if c.starts_with("token=") {
                Some(c[6..].to_string())
            } else {
                None
            }
        })
    });

    let token = match token {
        Some(t) => t,
        None => {
            return (StatusCode::UNAUTHORIZED, "missing token").into_response();
        }
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key".to_string());

    let claims = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data.claims,
        Err(e) => {
            return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
        }
    };

    req.extensions_mut().insert(claims.user_id);

    next.run(req).await
}
