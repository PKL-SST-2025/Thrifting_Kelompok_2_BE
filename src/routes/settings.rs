use actix_web::{get, post, web, HttpResponse, Scope};

use crate::{auth::AuthUser, error::{ApiError, ApiResult}, models::{LocaleSettings, UserProfile}, state::AppState};

#[get("/profile")]
pub async fn get_profile(state: web::Data<AppState>, user: AuthUser) -> ApiResult<HttpResponse> {
    let row: Option<UserProfile> = sqlx::query_as("SELECT user_id, name, phone FROM user_profiles WHERE user_id = $1")
        .bind(&user.0)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(row.unwrap_or(UserProfile{ user_id: user.0.clone(), name: None, phone: None })))
}

#[post("/profile")]
pub async fn set_profile(state: web::Data<AppState>, user: AuthUser, payload: web::Json<UserProfile>) -> ApiResult<HttpResponse> {
    let p = payload.into_inner();
    sqlx::query("INSERT INTO user_profiles (user_id, name, phone) VALUES ($1, $2, $3) ON CONFLICT(user_id) DO UPDATE SET name=excluded.name, phone=excluded.phone")
        .bind(&user.0)
        .bind(&p.name)
        .bind(&p.phone)
        .execute(&*state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/locale")]
pub async fn get_locale(state: web::Data<AppState>, user: AuthUser) -> ApiResult<HttpResponse> {
    let row: Option<LocaleSettings> = sqlx::query_as("SELECT user_id, country, language, currency, timezone FROM locale_settings WHERE user_id = $1")
        .bind(&user.0)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(row.unwrap_or(LocaleSettings{ user_id: user.0.clone(), country: None, language: None, currency: None, timezone: None })))
}

#[post("/locale")]
pub async fn set_locale(state: web::Data<AppState>, user: AuthUser, payload: web::Json<LocaleSettings>) -> ApiResult<HttpResponse> {
    let s = payload.into_inner();
    sqlx::query("INSERT INTO locale_settings (user_id, country, language, currency, timezone) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(user_id) DO UPDATE SET country=excluded.country, language=excluded.language, currency=excluded.currency, timezone=excluded.timezone")
        .bind(&user.0)
        .bind(&s.country)
        .bind(&s.language)
        .bind(&s.currency)
        .bind(&s.timezone)
        .execute(&*state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn scope() -> Scope { web::scope("/settings").service(get_profile).service(set_profile).service(get_locale).service(set_locale) }
