use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Entry {
    pub id: String,
    pub tome_id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}