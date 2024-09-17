use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use super::handler;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/food_items", post(handler::create_food_item))
        .route("/food_items/:id", get(handler::get_food_item))
        .with_state(pool)
}