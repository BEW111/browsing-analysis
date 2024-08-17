use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Deserialize, Serialize)]
struct TabUpdateEvent {
    timestamp: DateTime<Utc>,
    tab_id: String,
    url: String,
    title: String,
    type_of_visit: String,
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn log_tab_update_event(
    State(pool): State<PgPool>,
    Json(tab_update_event): Json<TabUpdateEvent>,
) -> Result<Json<TabUpdateEvent>, (StatusCode, String)> {
    let result = sqlx::query(
        r#"
        INSERT INTO events (timestamp, tab_id, url, title, type_of_visit) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING *
        "#,
    )
    .bind(&tab_update_event.timestamp)
    .bind(&tab_update_event.tab_id)
    .bind(&tab_update_event.url)
    .bind(&tab_update_event.title)
    .bind(&tab_update_event.type_of_visit)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(_) => {
            println!("Successfully uploaded event");
            Ok(Json(tab_update_event))
        }
        Err(e) => {
            println!("Failed to upload event");
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

async fn create_events_table(
    State(pool): State<PgPool>,
) -> Result<&'static str, (StatusCode, String)> {
    let query = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS events (
            id SERIAL PRIMARY KEY,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            tab_id TEXT NOT NULL,
            url TEXT NOT NULL,
            title TEXT NOT NULL,
            type_of_visit TEXT NOT NULL
        )
        "#,
    );

    match query.execute(&pool).await {
        Ok(_) => Ok("Table 'events' created successfully"),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/log_event", post(log_tab_update_event))
        .route("/create_table", post(create_events_table))
        .with_state(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@db/mydatabase")
        .await?;

    let app = create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
