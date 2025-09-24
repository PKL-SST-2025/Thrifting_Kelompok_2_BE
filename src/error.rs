use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found")] NotFound,
    #[error("Unauthorized")] Unauthorized,
    #[error("Forbidden")] Forbidden,
    #[error("Bad request: {0}")] BadRequest(String),
    #[error("Internal server error")] Internal,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().json(ErrorBody { error: "not_found" }),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(ErrorBody { error: "unauthorized" }),
            ApiError::Forbidden => HttpResponse::Forbidden().json(ErrorBody { error: "forbidden" }),
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({"error":"bad_request","message":msg}))
            }
            ApiError::Internal => HttpResponse::InternalServerError().json(ErrorBody { error: "internal" }),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
