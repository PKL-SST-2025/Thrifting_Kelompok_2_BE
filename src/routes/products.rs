use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use nanoid::nanoid;
use serde::Deserialize;

use crate::{auth::AuthUser, error::{ApiError, ApiResult}, models::{Product, ProductCreate, ProductUpdate}, state::AppState};
use sqlx::QueryBuilder;

#[derive(Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
    pub gender: Option<String>,
    pub size: Option<String>,
    pub color: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
}

#[get("")]
pub async fn list(state: web::Data<AppState>, q: web::Query<ProductQuery>) -> ApiResult<HttpResponse> {
    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new("SELECT * FROM products WHERE 1=1");
    if let Some(ref c) = q.category { qb.push(" AND category = ").push_bind(c); }
    if let Some(ref g) = q.gender { qb.push(" AND gender = ").push_bind(g); }
    if let Some(ref s) = q.size { qb.push(" AND size = ").push_bind(s); }
    if let Some(ref c) = q.color { qb.push(" AND color = ").push_bind(c); }
    if let Some(ref st) = q.status { qb.push(" AND status = ").push_bind(st); }

    if let Some(ref sort) = q.sort {
        match sort.as_str() {
            "price_asc" => { qb.push(" ORDER BY price_cents ASC"); }
            "price_desc" => { qb.push(" ORDER BY price_cents DESC"); }
            _ => { qb.push(" ORDER BY created_at DESC"); }
        }
    } else {
        qb.push(" ORDER BY created_at DESC");
    }

    let rows: Vec<Product> = qb
        .build_query_as()
        .fetch_all(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/{id}")]
pub async fn get_one(state: web::Data<AppState>, path: web::Path<String>) -> ApiResult<HttpResponse> {
    let id = path.into_inner();
    let row: Option<Product> = sqlx::query_as("SELECT * FROM products WHERE id = $1")
        .bind(&id)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    match row { Some(p) => Ok(HttpResponse::Ok().json(p)), None => Err(ApiError::NotFound) }
}

#[post("")]
pub async fn create(state: web::Data<AppState>, _user: AuthUser, payload: web::Json<ProductCreate>) -> ApiResult<HttpResponse> {
    let id = nanoid!();
    sqlx::query("INSERT INTO products (id, title, description, price_cents, category, color, size, gender, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active')")
        .bind(&id)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(payload.price_cents)
        .bind(&payload.category)
        .bind(&payload.color)
        .bind(&payload.size)
        .bind(&payload.gender)
        .execute(&*state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

#[put("/{id}")]
pub async fn update(state: web::Data<AppState>, _user: AuthUser, path: web::Path<String>, payload: web::Json<ProductUpdate>) -> ApiResult<HttpResponse> {
    let id = path.into_inner();
    let current: Option<Product> = sqlx::query_as("SELECT * FROM products WHERE id = $1")
        .bind(&id)
        .fetch_optional(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    let Some(curr) = current else { return Err(ApiError::NotFound) };

    let title = payload.title.clone().unwrap_or(curr.title);
    let description = payload.description.clone().unwrap_or(curr.description);
    let price = payload.price_cents.unwrap_or(curr.price_cents);
    let category = payload.category.clone().unwrap_or(curr.category);
    let color = payload.color.clone().or(curr.color);
    let size = payload.size.clone().or(curr.size);
    let gender = payload.gender.clone().or(curr.gender);
    let status = payload.status.clone().unwrap_or(curr.status);

    sqlx::query("UPDATE products SET title = $1, description = $2, price_cents = $3, category = $4, color = $5, size = $6, gender = $7, status = $8 WHERE id = $9")
        .bind(&title)
        .bind(&description)
        .bind(price)
        .bind(&category)
        .bind(&color)
        .bind(&size)
        .bind(&gender)
        .bind(&status)
        .bind(&id)
        .execute(&*state.pool)
    .await
    .map_err(|_| ApiError::Internal)?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/{id}")]
pub async fn delete(state: web::Data<AppState>, _user: AuthUser, path: web::Path<String>) -> ApiResult<HttpResponse> {
    let id = path.into_inner();
    let res = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(&id)
        .execute(&*state.pool)
        .await
        .map_err(|_| ApiError::Internal)?;
    if res.rows_affected() == 0 { return Err(ApiError::NotFound); }
    Ok(HttpResponse::NoContent().finish())
}

pub fn scope() -> Scope {
    web::scope("/products")
        .service(list)
        .service(get_one)
        .service(create)
        .service(update)
        .service(delete)
}
