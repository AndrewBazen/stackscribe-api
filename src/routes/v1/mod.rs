use axum::{Router};
use sqlx::PgPool;

pub mod user;
pub mod archive;
pub mod tome;
pub mod entry;
pub mod sync;

pub fn create_v1_routes(pool: PgPool) -> Router {
    Router::new()
        .nest("/users", user::routes(pool.clone()))
        .nest("/archives", archive::routes(pool.clone()))
        .nest("/tomes", tome::routes(pool.clone()))
        .nest("/entries", entry::routes(pool.clone()))
        .nest("/sync", sync::create_sync_routes(pool))
}
