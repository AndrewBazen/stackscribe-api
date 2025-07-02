mod models;
mod routes;


use axum::{Router};
use std::net::SocketAddr;
use tracing_subscriber;
use dotenvy::dotenv;
use axum::Server;
use sqlx::postgres::PgPoolOptions;
use crate::routes::v1::create_v1_routes;

// Temporary user ID - replace with proper authentication
const TEMP_USER_ID: &str = "temp-user-123";

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

async fn not_found_handler() -> impl IntoResponse {
    let body = Json(json!({
        "error": "Route not found",
        "status": 404
    }));
    (StatusCode::NOT_FOUND, body)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();  
    // Load the database URL from environment variables
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment variables");
    // Create a connection pool for the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Migrations completed");

    let _ = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash) 
         VALUES ($1, $2, $3, $4) 
         ON CONFLICT (id) DO NOTHING"
    ).bind(TEMP_USER_ID)
     .bind("test_user")
     .bind("testuser@example.com")
     .bind("hashed_password")
     .execute(&pool)
     .await;

    let app = Router::new()
        .nest("/api/v1", create_v1_routes(pool.clone()))
        .fallback(not_found_handler);

    // Define the address to listen on
    // You can change the port or address as needed
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    // Start the server
    tracing::info!("Starting server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");

    Ok(())
}
