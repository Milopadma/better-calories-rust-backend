use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use tracing::debug;
use std::env;

use super::model::{User, CreateUserRequest};

pub async fn create_user(pool: &PgPool, payload: CreateUserRequest) -> Result<User, sqlx::Error> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Hashing password for new user");
    }
    let password_hash = hash(payload.password, DEFAULT_COST).expect("Failed to hash password");

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Inserting new user into database");
    }
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        payload.username,
        payload.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("User inserted successfully: {:?}", user);
    }
    Ok(user)
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Fetching all users from database");
    }
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await?;

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Retrieved {} users", users.len());
    }
    Ok(users)
}

pub async fn get_user(pool: &PgPool, id: i64) -> Result<User, sqlx::Error> {
    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Fetching user with id {} from database", id);
    }
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    if env::var("DEBUG").unwrap_or_default() == "true" {
        debug!("Retrieved user: {:?}", user);
    }
    Ok(user)
}