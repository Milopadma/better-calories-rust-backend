use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: i64,
    pub name: String,
    pub calories: i32,
    pub protein: i32,
    pub carbohydrate: i32,
    pub fat: i32,
    pub sugar: i32,
}