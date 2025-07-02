use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use crate::models::archive::Archive;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArchivePayload {
    pub name: String,
    pub description: String,
}

pub async fn list_archives(
    State(pool): State<PgPool>
) -> Result<Json<Vec<Archive>>, axum::http::StatusCode> {
    // This function retrieves all archives from the database
    // and returns them as a JSON response.
    let archives = sqlx::query_as::<_, Archive>("SELECT * FROM archives")
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(archives))
}

pub async fn create_archive(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateArchivePayload>
) -> Result<Json<Archive>, axum::http::StatusCode> {
    // This function creates a new archive in the database
    // using the provided JSON payload and returns the created archive.
    let new_archive = sqlx::query_as::<_, Archive>(
        "INSERT INTO archives (id, user_id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING *"
    )
    .bind(format!("archive-{}", uuid::Uuid::new_v4()))
    .bind("temp-user-123") // Use temporary user ID
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(new_archive))
}

pub async fn update_archive(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Json(payload): Json<Archive>
) -> Result<Json<Archive>, axum::http::StatusCode> {
    // This function updates an existing archive in the database
    // using the provided ID and JSON payload, returning the updated archive.
    let updated_archive = sqlx::query_as::<_, Archive>(
        "UPDATE archives SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(updated_archive))
}

pub async fn get_archive(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<Json<Archive>, axum::http::StatusCode> {
    // This function retrieves a specific archive by its ID from the database
    // and returns it as a JSON response.
    let archive = sqlx::query_as::<_, Archive>("SELECT * FROM archives WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(archive))
}

pub async fn delete_archive(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<(), axum::http::StatusCode> {
    // This function deletes an archive by its ID from the database.
    sqlx::query("DELETE FROM archives WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(())
}