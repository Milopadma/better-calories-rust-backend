mod config;
mod db;
mod error;
mod features;
mod routes;

use std::net::SocketAddr;
use sqlx::migrate::MigrateDatabase;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dotenv().ok(); // This line loads the .env file

    // Load configuration
    let config = config::load_config();

    // Ensure database exists
    if !sqlx::Postgres::database_exists(&config).await.unwrap_or(false) {
        sqlx::Postgres::create_database(&config).await.expect("Failed to create database");
    }

    // Set up database connection
    let pool = db::setup_database(&config).await.expect("Failed to set up database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Set up routes
    let app = create_router(pool);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}