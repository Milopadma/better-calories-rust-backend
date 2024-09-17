use serde::Deserialize;
use sqlx::PgPool;

use super::model::Meal;

#[derive(Deserialize)]
pub struct CreateMeal {
    pub day_id: i64,
    pub name: String,
}

pub async fn create_meal(pool: &PgPool, payload: CreateMeal) -> Result<Meal, sqlx::Error> {
    sqlx::query_as!(
        Meal,
        "INSERT INTO meals (day_id, name) VALUES ($1, $2) RETURNING *",
        payload.day_id,
        payload.name
    )
    .fetch_one(pool)
    .await
}

pub async fn get_meal(pool: &PgPool, id: i64) -> Result<Meal, sqlx::Error> {
    sqlx::query_as!(Meal, "SELECT * FROM meals WHERE id = $1", id)
        .fetch_one(pool)
        .await
}