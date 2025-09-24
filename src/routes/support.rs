use actix_web::{post, web, HttpResponse, Scope};
use serde::Serialize;

use crate::{error::ApiResult, models::SupportMessagePayload};

#[derive(Serialize)]
struct SupportResponse { ok: bool }

#[post("/message")]
pub async fn message(payload: web::Json<SupportMessagePayload>) -> ApiResult<HttpResponse> {
    // For now, just log. In production, forward to email/ticketing.
    tracing::info!("Support message from {} <{}>: {}", payload.name, payload.email, payload.subject);
    Ok(HttpResponse::Ok().json(SupportResponse { ok: true }))
}

pub fn scope() -> Scope { web::scope("/support").service(message) }
