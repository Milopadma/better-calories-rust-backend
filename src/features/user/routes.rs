use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use super::handler;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(handler::create_user))
        .route("/users", get(handler::get_users))
        .route("/users/:id", get(handler::get_user))
        .with_state(pool)
}