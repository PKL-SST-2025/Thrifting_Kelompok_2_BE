mod server;
mod state;
mod error;
mod auth;
mod routes;
mod models;
mod schema;

use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Server::run().await
}
