use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Day {
    pub id: i64,
    pub user_id: i64,
    pub date: NaiveDate,
}