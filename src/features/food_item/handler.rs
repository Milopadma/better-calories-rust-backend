use axum::{extract::Path, http::StatusCode, Json};
use sqlx::PgPool;

use super::model::FoodItem;
use super::service;

pub async fn create_food_item(
    pool: axum::extract::State<PgPool>,
    Json(payload): Json<service::CreateFoodItem>,
) -> Result<(StatusCode, Json<FoodItem>), StatusCode> {
    let food_item = service::create_food_item(&pool, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(food_item)))
}

pub async fn get_food_item(
    pool: axum::extract::State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<FoodItem>, StatusCode> {
    let food_item = service::get_food_item(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(food_item))
}