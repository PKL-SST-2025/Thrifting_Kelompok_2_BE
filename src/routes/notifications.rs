use actix_web::{get, post, web, HttpResponse, Scope};

use crate::{auth::AuthUser, error::{ApiError, ApiResult}, models::NotificationSettings, state::AppState};

#[get("/settings")]
pub async fn get_settings(state: web::Data<AppState>, user: AuthUser) -> ApiResult<HttpResponse> {
    let row: Option<NotificationSettings> = sqlx::query_as("SELECT user_id, email, new_arrivals, promotions, order_updates FROM notification_settings WHERE user_id = $1")
        .bind(&user.0)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;

    let mut s = row.unwrap_or_else(|| NotificationSettings { user_id: user.0.clone(), ..Default::default() });
    s.user_id = user.0.clone();
    Ok(HttpResponse::Ok().json(s))
}

#[post("/settings")]
pub async fn set_settings(state: web::Data<AppState>, user: AuthUser, payload: web::Json<NotificationSettings>) -> ApiResult<HttpResponse> {
    let s = payload.into_inner();
    sqlx::query(
        "INSERT INTO notification_settings (user_id, email, new_arrivals, promotions, order_updates) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT(user_id) DO UPDATE SET email=excluded.email, new_arrivals=excluded.new_arrivals, promotions=excluded.promotions, order_updates=excluded.order_updates"
    )
    .bind(&user.0)
    .bind(s.email)
    .bind(s.new_arrivals)
    .bind(s.promotions)
    .bind(s.order_updates)
    .execute(&*state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn scope() -> Scope { web::scope("/notifications").service(get_settings).service(set_settings) }
