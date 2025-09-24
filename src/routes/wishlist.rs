use actix_web::{delete, get, post, web, HttpResponse, Scope};

use crate::{auth::AuthUser, error::{ApiError, ApiResult}, models::WishlistItem, state::AppState};

#[get("")]
pub async fn list(state: web::Data<AppState>, user: AuthUser) -> ApiResult<HttpResponse> {
    let rows = sqlx::query_as::<_, WishlistItem>("SELECT * FROM wishlist WHERE user_id = $1 ORDER BY added_at DESC")
        .bind(&user.0)
        .fetch_all(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(rows))
}

#[post("/{product_id}")]
pub async fn add(state: web::Data<AppState>, user: AuthUser, path: web::Path<String>) -> ApiResult<HttpResponse> {
    let product_id = path.into_inner();
    sqlx::query("INSERT INTO wishlist (user_id, product_id) VALUES ($1, $2) ON CONFLICT (user_id, product_id) DO NOTHING")
        .bind(&user.0)
        .bind(&product_id)
        .execute(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/{product_id}")]
pub async fn remove(state: web::Data<AppState>, user: AuthUser, path: web::Path<String>) -> ApiResult<HttpResponse> {
    let product_id = path.into_inner();
    sqlx::query("DELETE FROM wishlist WHERE user_id = $1 AND product_id = $2")
        .bind(&user.0)
        .bind(&product_id)
        .execute(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn scope() -> Scope { web::scope("/wishlist").service(list).service(add).service(remove) }
