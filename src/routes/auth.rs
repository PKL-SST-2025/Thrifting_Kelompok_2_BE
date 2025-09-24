use actix_web::{post, web, HttpResponse, Scope};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use nanoid::nanoid;

use crate::{auth::create_jwt, error::{ApiError, ApiResult}, models::{AuthResponse, LoginPayload, RegisterPayload}, state::AppState};

#[post("/register")]
pub async fn register(state: web::Data<AppState>, payload: web::Json<RegisterPayload>) -> ApiResult<HttpResponse> {
    let id = nanoid!();
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| ApiError::Internal)?
        .to_string();

    sqlx::query("INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, $3, NULL)")
        .bind(&id)
        .bind(&payload.email)
        .bind(&hash)
        .execute(&*state.pool)
    .await
    .map_err(|e| ApiError::BadRequest(format!("email already used: {e}")))?;

    Ok(HttpResponse::Created().finish())
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, payload: web::Json<LoginPayload>) -> ApiResult<HttpResponse> {
    let row = sqlx::query_as::<_, (String, String)>("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;

    let Some((user_id, password_hash)) = row else { return Err(ApiError::Unauthorized) };

    let parsed = PasswordHash::new(&password_hash).map_err(|_| ApiError::Internal)?;
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .is_ok()
    {
        let token = create_jwt(&user_id, &state.jwt_secret, 60 * 60 * 24 * 7)?; // 7 days
        return Ok(HttpResponse::Ok().json(AuthResponse { token }));
    }

    Err(ApiError::Unauthorized)
}

pub fn scope() -> Scope {
    web::scope("/auth").service(register).service(login)
}
