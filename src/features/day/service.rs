use serde::Deserialize;
use sqlx::PgPool;
use chrono::NaiveDate;

use super::model::Day;

#[derive(Deserialize)]
pub struct CreateDay {
    pub user_id: i64,
    pub date: NaiveDate,
}

pub async fn create_day(pool: &PgPool, payload: CreateDay) -> Result<Day, sqlx::Error> {
    sqlx::query_as!(
        Day,
        "INSERT INTO days (user_id, date) VALUES ($1, $2) RETURNING *",
        payload.user_id,
        payload.date
    )
    .fetch_one(pool)
    .await
}

pub async fn get_day(pool: &PgPool, id: i64) -> Result<Day, sqlx::Error> {
    sqlx::query_as!(Day, "SELECT * FROM days WHERE id = $1", id)
        .fetch_one(pool)
        .await
}