use axum::{extract::Path, http::StatusCode, Json};
use sqlx::PgPool;

use super::model::Day;
use super::service;

pub async fn create_day(
    pool: axum::extract::State<PgPool>,
    Json(payload): Json<service::CreateDay>,
) -> Result<(StatusCode, Json<Day>), StatusCode> {
    let day = service::create_day(&pool, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(day)))
}

pub async fn get_day(
    pool: axum::extract::State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Day>, StatusCode> {
    let day = service::get_day(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(day))
}