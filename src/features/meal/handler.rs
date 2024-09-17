use axum::{extract::Path, http::StatusCode, Json};
use sqlx::PgPool;

use super::model::Meal;
use super::service;

pub async fn create_meal(
    pool: axum::extract::State<PgPool>,
    Json(payload): Json<service::CreateMeal>,
) -> Result<(StatusCode, Json<Meal>), StatusCode> {
    let meal = service::create_meal(&pool, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(meal)))
}

pub async fn get_meal(
    pool: axum::extract::State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Meal>, StatusCode> {
    let meal = service::get_meal(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(meal))
}