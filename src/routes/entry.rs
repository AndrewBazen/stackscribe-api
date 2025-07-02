use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use crate::models::entry::Entry;

pub async fn list_entries( 
    State(pool): State<PgPool>
) -> Result<Json<Vec<Entry>>, axum::http::StatusCode> {
    // This function retrieves all entries from the database
    // and returns them as a JSON response.
    let entries = sqlx::query_as::<_, Entry>("SELECT * FROM entries")
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(entries))
}

pub async fn create_entry(
    State(pool): State<PgPool>,
    Json(payload): Json<Entry>
) -> Result<Json<Entry>, axum::http::StatusCode> {
    // This function creates a new entry in the database
    // using the provided JSON payload and returns the created entry.
    let new_entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (id, tome_id, user_id, title, content, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING *"
    )
    .bind(&payload.id)
    .bind(&payload.tome_id)
    .bind(&payload.user_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(new_entry))
}

pub async fn update_entry(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Json(payload): Json<Entry>
) -> Result<Json<Entry>, axum::http::StatusCode> {
    // This function updates an existing entry in the database
    // using the provided ID and JSON payload, returning the updated entry.
    let updated_entry = sqlx::query_as::<_, Entry>(
        "UPDATE entries SET title = $1, content = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(updated_entry))
}

pub async fn get_entry(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<Json<Entry>, axum::http::StatusCode> {
    // This function retrieves a specific entry by its ID from the database
    // and returns it as a JSON response.
    let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(entry))
}

pub async fn delete_entry(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<Json<Entry>, axum::http::StatusCode> {
    // This function deletes a specific entry by its ID from the database
    // and returns the deleted entry as a JSON response.
    let deleted_entry = sqlx::query_as::<_, Entry>("DELETE FROM entries WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(deleted_entry))
}