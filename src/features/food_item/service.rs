use serde::Deserialize;
use sqlx::PgPool;

use super::model::FoodItem;

#[derive(Deserialize)]
pub struct CreateFoodItem {
    pub name: String,
    pub calories: i32,
    pub protein: i32,
    pub carbohydrate: i32,
    pub fat: i32,
    pub sugar: i32,
}

pub async fn create_food_item(pool: &PgPool, payload: CreateFoodItem) -> Result<FoodItem, sqlx::Error> {
    sqlx::query_as!(
        FoodItem,
        "INSERT INTO food_items (name, calories, protein, carbohydrate, fat, sugar) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        payload.name,
        payload.calories,
        payload.protein,
        payload.carbohydrate,
        payload.fat,
        payload.sugar
    )
    .fetch_one(pool)
    .await
}

pub async fn get_food_item(pool: &PgPool, id: i64) -> Result<FoodItem, sqlx::Error> {
    sqlx::query_as!(FoodItem, "SELECT * FROM food_items WHERE id = $1", id)
        .fetch_one(pool)
        .await
}