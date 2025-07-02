use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tome {
    pub id: String,
    pub archive_id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}   