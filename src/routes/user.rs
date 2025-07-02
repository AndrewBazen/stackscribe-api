use axum::{Json, extract::Path, extract::State};
use sqlx::PgPool;
use crate::models::user::User;

pub async fn list_users(
    State(pool): State<PgPool>
) -> Result<Json<Vec<User>>, axum::http::StatusCode> {
    // This function retrieves all users from the database
    // and returns them as a JSON response.
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<User>
) -> Result<Json<User>, axum::http::StatusCode> {
    // This function creates a new user in the database
    // using the provided JSON payload and returns the created user.
    let new_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email, password_hash, created_at, updated_at, is_active) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true) RETURNING *"
    )
    .bind(&payload.id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&payload.password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(new_user))
}

pub async fn update_user(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
    Json(payload): Json<User>
) -> Result<Json<User>, axum::http::StatusCode> {
    // This function updates an existing user in the database
    // using the provided ID and JSON payload, returning the updated user.
    let updated_user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, email = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(updated_user))
}

pub async fn get_user(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<Json<User>, axum::http::StatusCode> {
    // This function retrieves a user by their ID from the database.
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    Ok(Json(user))
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(pool): State<PgPool>
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    // This function deletes a user by their ID from the database.
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(axum::http::StatusCode::NO_CONTENT)
}
