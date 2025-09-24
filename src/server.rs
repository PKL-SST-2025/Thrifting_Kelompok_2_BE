use std::net::SocketAddr;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::routes;

use crate::state::AppState;

pub struct Server;

impl Server {
    pub async fn run() -> std::io::Result<()> {
        // logging
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx=warn,actix_web=info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // load env
        dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/thrifting".into());
        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".into());
        let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect DB");

        // run migrations (sqlx will look in ./migrations)
        if let Err(e) = sqlx::migrate!().run(&pool).await {
            eprintln!("Migration error: {e}");
        }

        let state = AppState::new(pool);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        tracing::info!("Starting server on http://{}", addr);

        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_method()
                .allow_any_header()
                .allowed_origin(&cors_origin)
                .supports_credentials();

            App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .app_data(web::Data::new(state.clone()))
                .configure(routes::init)
        })
        .bind(addr)?
        .run()
        .await
    }
}
