use sqlx::types::BigDecimal;
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(FromRow, Debug)]
pub struct Day {
    pub id: i64,
    pub user_id: i64,
    pub date: NaiveDate,
}

#[derive(FromRow, Debug)]
pub struct Meal {
    pub id: i64,
    pub day_id: i64,
    pub name: String,
}

#[derive(FromRow, Debug)]
pub struct FoodItem {
    pub id: i64,
    pub name: String,
    pub calories: i32,
    pub protein: i32,
    pub carbohydrate: i32,
    pub fat: i32,
    pub sugar: i32,
}

#[derive(FromRow, Debug)]
pub struct MealFoodItem {
    pub id: i64,
    pub meal_id: i64,
    pub food_item_id: i64,
    pub quantity: i32,
}