use actix_web::{get, web, HttpResponse, Scope};

use crate::{error::{ApiError, ApiResult}, models::FaqItem, state::AppState};

#[get("")]
pub async fn list(state: web::Data<AppState>) -> ApiResult<HttpResponse> {
    let rows = sqlx::query_as::<_, FaqItem>("SELECT id, question, answer FROM faqs ORDER BY id ASC")
        .fetch_all(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(rows))
}

pub fn scope() -> Scope { web::scope("/faq").service(list) }
