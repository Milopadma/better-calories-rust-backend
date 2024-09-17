use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tracing::debug;
use std::env;

use super::handler;

pub fn create_routes(pool: PgPool) -> Router {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Creating user routes");
    }
    Router::new()
        .route("/", post(handler::create_user))
        .route("/", get(handler::get_users))
        .route("/:id", get(handler::get_user))
        .with_state(pool)
}