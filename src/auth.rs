use actix_web::{dev::Payload, error::ErrorUnauthorized, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, secret: &str, exp_secs: i64) -> Result<String, ApiError> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(exp_secs))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| ApiError::Internal)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, ApiError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;
    Ok(data.claims)
}

#[derive(Clone)]
pub struct AuthUser(pub String); // user_id

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(state) = req.app_data::<actix_web::web::Data<AppState>>() {
            let secret = state.jwt_secret.clone();
            if let Some(auth) = req.headers().get("authorization") {
                if let Ok(auth_str) = auth.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        match verify_jwt(token, &secret) {
                            Ok(claims) => return ready(Ok(AuthUser(claims.sub))),
                            Err(_) => return ready(Err(ErrorUnauthorized("invalid_token"))),
                        }
                    }
                }
            }
        }
        ready(Err(ErrorUnauthorized("missing_token")))
    }
}
