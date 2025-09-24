use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change_me_super_secret".into());
        Self {
            pool: Arc::new(pool),
            jwt_secret: Arc::new(jwt_secret),
        }
    }
}
