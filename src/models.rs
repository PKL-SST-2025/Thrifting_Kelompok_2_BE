use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Users
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

// Products
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price_cents: i64,
    pub category: String,
    pub color: Option<String>,
    pub size: Option<String>,
    pub gender: Option<String>,
    pub status: String, // active/sold
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ProductCreate {
    pub title: String,
    pub description: String,
    pub price_cents: i64,
    pub category: String,
    pub color: Option<String>,
    pub size: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProductUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_cents: Option<i64>,
    pub category: Option<String>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub gender: Option<String>,
    pub status: Option<String>,
}

// Wishlist
#[derive(Debug, Serialize, FromRow)]
pub struct WishlistItem {
    pub user_id: String,
    pub product_id: String,
    pub added_at: DateTime<Utc>,
}

// Notifications settings
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NotificationSettings {
    pub user_id: String,
    pub email: bool,
    pub new_arrivals: bool,
    pub promotions: bool,
    pub order_updates: bool,
}

// Profile / Locale
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct LocaleSettings {
    pub user_id: String,
    pub country: Option<String>,
    pub language: Option<String>,
    pub currency: Option<String>,
    pub timezone: Option<String>,
}

// Static content
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct StoreLocation {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct SupportMessagePayload {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct FaqItem {
    pub id: i64,
    pub question: String,
    pub answer: String,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            user_id: "".into(),
            email: true,
            new_arrivals: true,
            promotions: true,
            order_updates: true,
        }
    }
}
