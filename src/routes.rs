use axum::Router;
use sqlx::PgPool;

use crate::features::{user, day, meal, food_item};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .merge(user::routes::create_routes(pool.clone()))
        .merge(day::routes::create_routes(pool.clone()))
        .merge(meal::routes::create_routes(pool.clone()))
        .merge(food_item::routes::create_routes(pool))
}