use axum::{extract::Path, http::StatusCode, Json};
use sqlx::PgPool;

use super::model::User;
use super::service;

pub async fn create_user(
    pool: axum::extract::State<PgPool>,
    Json(payload): Json<service::CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = service::create_user(&pool, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_users(
    pool: axum::extract::State<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = service::get_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user(
    pool: axum::extract::State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = service::get_user(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}