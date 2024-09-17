use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Meal {
    pub id: i64,
    pub day_id: i64,
    pub name: String,
}