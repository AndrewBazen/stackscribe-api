use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Archive {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}