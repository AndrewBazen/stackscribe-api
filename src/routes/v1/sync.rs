use axum::{Router, routing::{get, post}, Json, http::StatusCode, extract::{Query, State}};
use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};

// Temporary user ID - replace with proper authentication
const TEMP_USER_ID: &str = "temp-user-123";

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncArchive {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncTome {
    pub id: String,
    pub archive_id: String,
    pub name: String, // Note: database uses 'name' field for tomes
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncEntry {
    pub id: String,
    pub tome_id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub archives: Vec<SyncArchive>,
    pub tomes: Vec<SyncTome>,
    pub entries: Vec<SyncEntry>,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncRequest {
    pub archives: Vec<SyncArchive>,
    pub tomes: Vec<SyncTome>,
    pub entries: Vec<SyncEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SyncQuery {
    pub since: Option<String>,
}

async fn get_sync(
    Query(params): Query<SyncQuery>,
    State(pool): State<PgPool>,
) -> Result<Json<SyncResponse>, StatusCode> {
    // Parse the since parameter or use a very old timestamp as default
    let since_timestamp: DateTime<Utc> = match params.since {
        Some(since_str) => since_str.parse().unwrap_or_else(|_| {
            DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap().with_timezone(&Utc)
        }),
        None => DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
    };

    let archives = sqlx::query(
        "SELECT id, name, description, created_at, updated_at 
         FROM archives 
         WHERE user_id = $1 AND updated_at > $2
         ORDER BY updated_at DESC"
    )
    .bind(TEMP_USER_ID)
    .bind(since_timestamp)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in archives query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| SyncArchive {
        id: row.get::<String, _>("id"),
        name: row.get("name"),
        description: row.get("description"),
        created_at: row.get::<NaiveDateTime, _>("created_at").and_utc().to_rfc3339(),
        updated_at: row.get::<NaiveDateTime, _>("updated_at").and_utc().to_rfc3339(),
    })
    .collect();

    let tomes = sqlx::query(
        "SELECT id, archive_id, name, description, created_at, updated_at 
         FROM tomes 
         WHERE user_id = $1 AND updated_at > $2
         ORDER BY updated_at DESC"
    )
    .bind(TEMP_USER_ID)
    .bind(since_timestamp)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in tomes query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| SyncTome {
        id: row.get::<String, _>("id"),
        archive_id: row.get("archive_id"),
        name: row.get("name"),
        description: row.get("description"),
        created_at: row.get::<NaiveDateTime, _>("created_at").and_utc().to_rfc3339(),
        updated_at: row.get::<NaiveDateTime, _>("updated_at").and_utc().to_rfc3339(),
    })
    .collect();

    let entries = sqlx::query(
        "SELECT id, tome_id, title, content, created_at, updated_at 
         FROM entries 
         WHERE user_id = $1 AND updated_at > $2
         ORDER BY updated_at DESC"
    )
    .bind(TEMP_USER_ID)
    .bind(since_timestamp)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in entries query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| SyncEntry {
        id: row.get::<String, _>("id"),
        tome_id: row.get("tome_id"),
        title: row.get("title"),
        content: row.get("content"),
        created_at: row.get::<NaiveDateTime, _>("created_at").and_utc().to_rfc3339(),
        updated_at: row.get::<NaiveDateTime, _>("updated_at").and_utc().to_rfc3339(),
    })
    .collect();

    let last_modified = Utc::now().to_rfc3339();

    Ok(Json(SyncResponse {
        archives,
        tomes,
        entries,
        last_modified,
    }))
}

async fn post_sync(
    State(pool): State<PgPool>,
    Json(payload): Json<SyncRequest>,
) -> Result<Json<HashMap<String, String>>, StatusCode> {
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to start transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Process archives
    for archive in payload.archives {
        let created_at: DateTime<Utc> = archive.created_at.parse().map_err(|e| {
            eprintln!("Failed to parse created_at for archive: {}", e);
            StatusCode::BAD_REQUEST
        })?;
        let updated_at: DateTime<Utc> = archive.updated_at.parse().map_err(|e| {
            eprintln!("Failed to parse updated_at for archive: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    
        sqlx::query(
            "INSERT INTO archives (id, user_id, name, description, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) 
             DO UPDATE SET 
                name = EXCLUDED.name, 
                description = EXCLUDED.description, 
                updated_at = EXCLUDED.updated_at
            WHERE archives.updated_at < EXCLUDED.updated_at"
        )
        .bind(&archive.id)
        .bind(TEMP_USER_ID)
        .bind(&archive.name)
        .bind(&archive.description)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert/update archive: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    // Process tomes
    for tome in payload.tomes {
        let created_at: DateTime<Utc> = tome.created_at.parse().map_err(|e| {
            eprintln!("Failed to parse created_at for tome: {}", e);
            StatusCode::BAD_REQUEST
        })?;
        let updated_at: DateTime<Utc> = tome.updated_at.parse().map_err(|e| {
            eprintln!("Failed to parse updated_at for tome: {}", e);
            StatusCode::BAD_REQUEST
        })?;
        
        sqlx::query(
            "INSERT INTO tomes (id, archive_id, user_id, name, description, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (id)
             DO UPDATE SET 
                name = EXCLUDED.name, 
                description = EXCLUDED.description, 
                updated_at = EXCLUDED.updated_at
             WHERE tomes.updated_at < EXCLUDED.updated_at"
        )
        .bind(&tome.id)
        .bind(&tome.archive_id)
        .bind(TEMP_USER_ID)
        .bind(&tome.name)
        .bind(&tome.description)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert/update tome: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    // Process entries
    for entry in payload.entries {
        let created_at: DateTime<Utc> = entry.created_at.parse().map_err(|e| {
            eprintln!("Failed to parse created_at for entry: {}", e);
            StatusCode::BAD_REQUEST
        })?;
        let updated_at: DateTime<Utc> = entry.updated_at.parse().map_err(|e| {
            eprintln!("Failed to parse updated_at for entry: {}", e);
            StatusCode::BAD_REQUEST
        })?;

        sqlx::query(
            "INSERT INTO entries (id, tome_id, user_id, title, content, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (id) 
             DO UPDATE SET 
                title = EXCLUDED.title, 
                content = EXCLUDED.content, 
                updated_at = EXCLUDED.updated_at
             WHERE entries.updated_at < EXCLUDED.updated_at"
        )
        .bind(&entry.id)
        .bind(&entry.tome_id)
        .bind(TEMP_USER_ID)
        .bind(&entry.title)
        .bind(&entry.content)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert/update entry: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let mut response = HashMap::new();
    response.insert("success".to_string(), "true".to_string());
    response.insert("lastModified".to_string(), Utc::now().to_rfc3339());

    Ok(Json(response))
}

pub fn create_sync_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_sync))
        .route("/", post(post_sync))
        .with_state(pool)
}