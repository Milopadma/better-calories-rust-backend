use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use tracing::{debug, error};
use std::env;

use super::model::{User, CreateUserRequest};
use super::service;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Attempting to create user: {:?}", payload);
    }
    match service::create_user(&pool, payload).await {
        Ok(user) => {
            if env::var("DEBUG").unwrap_or_default() == "true" {
                debug!("User created successfully: {:?}", user);
            }
            Ok((StatusCode::CREATED, Json(user)))
        },
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create user: {:?}", e)))
        }
    }
}

pub async fn get_users(
    pool: axum::extract::State<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Attempting to get all users");
    }
    let users = service::get_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Retrieved {} users", users.len());
    }
    Ok(Json(users))
}

pub async fn get_user(
    pool: axum::extract::State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Attempting to get user with id: {}", id);
    }
    let user = service::get_user(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Retrieved user: {:?}", user);
    }
    Ok(Json(user))
}