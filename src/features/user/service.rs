use serde::Deserialize;
use sqlx::PgPool;

use super::model::User;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn create_user(pool: &PgPool, payload: CreateUser) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        payload.username,
        payload.email,
        payload.password_hash
    )
    .fetch_one(pool)
    .await
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_user(pool: &PgPool, id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await
}