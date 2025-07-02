// src/routes/v1/user.rs
use axum::{Router, routing::get};
use sqlx::PgPool;
use crate::routes::entry::{
    create_entry, delete_entry, get_entry, list_entries, update_entry
};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_entries).post(create_entry))
        .route("/:id", get(get_entry).put(update_entry).delete(delete_entry))
        .with_state(pool)
}
