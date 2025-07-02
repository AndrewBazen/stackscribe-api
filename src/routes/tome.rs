use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use crate::models::tome::Tome;

pub async fn list_tomes(State(pool): State<PgPool>) -> Result<Json<Vec<Tome>>, axum::http::StatusCode> {
    // This function retrieves all tomes from the database
    // and returns them as a JSON response.
    let tomes = sqlx::query_as::<_, Tome>("SELECT * FROM tomes")
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(tomes))
}

pub async fn create_tome(
    State(pool): State<PgPool>,
    Json(payload): Json<Tome>,
) -> Result<Json<Tome>, axum::http::StatusCode> {
    // This function creates a new tome in the database
    // using the provided JSON payload and returns the created tome.
    let new_tome = sqlx::query_as::<_, Tome>(
        "INSERT INTO tomes (id, archive_id, user_id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING *"
    )
    .bind(&payload.id)
    .bind(&payload.archive_id)
    .bind(&payload.user_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(new_tome))
}

pub async fn update_tome(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Json(payload): Json<Tome>
) -> Result<Json<Tome>, axum::http::StatusCode> {
    // This function updates an existing tome in the database
    // using the provided ID and JSON payload, returning the updated tome.
    let updated_tome = sqlx::query_as::<_, Tome>(
        "UPDATE tomes SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(updated_tome))
}

pub async fn get_tome( 
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<Json<Tome>, axum::http::StatusCode> {
    // This function retrieves a specific tome by its ID from the database
    // and returns it as a JSON response.
    let tome = sqlx::query_as::<_, Tome>("SELECT * FROM tomes WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(tome))
}

pub async fn delete_tome(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<(), axum::http::StatusCode> {
    // This function deletes a tome by its ID from the database.
    sqlx::query("DELETE FROM tomes WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(())
}