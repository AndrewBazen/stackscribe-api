// src/routes/v1/user.rs
use axum::{Router, routing::get};
use sqlx::PgPool;
use crate::routes::archive::{
    create_archive, delete_archive, get_archive, list_archives, update_archive
};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_archives).post(create_archive))
        .route("/:id", get(get_archive).put(update_archive).delete(delete_archive))
        .with_state(pool)
}
