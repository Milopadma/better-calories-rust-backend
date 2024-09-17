use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use super::handler;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/meals", post(handler::create_meal))
        .route("/meals/:id", get(handler::get_meal))
        .with_state(pool)
}