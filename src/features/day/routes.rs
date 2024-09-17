use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use super::handler;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/days", post(handler::create_day))
        .route("/days/:id", get(handler::get_day))
        .with_state(pool)
}