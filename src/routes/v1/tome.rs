// src/routes/v1/user.rs
use axum::{Router, routing::get};
use sqlx::PgPool;
use crate::routes::tome::{
    create_tome, delete_tome, get_tome, list_tomes, update_tome
};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_tomes).post(create_tome))
        .route("/:id", get(get_tome).put(update_tome).delete(delete_tome))
        .with_state(pool)
}
