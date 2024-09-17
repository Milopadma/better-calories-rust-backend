mod config;
mod db;
mod error;
mod features;
mod routes;

use std::net::SocketAddr;
use sqlx::migrate::MigrateDatabase;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
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
    let app = routes::create_router(pool);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}