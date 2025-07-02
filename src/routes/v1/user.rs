// src/routes/v1/user.rs
use axum::{Router, routing::get};
use sqlx::PgPool;
use crate::routes::user::{
    create_user, delete_user, get_user, list_users, update_user
};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool)
}
