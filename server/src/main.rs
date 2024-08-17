use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use dotenv::{dotenv, var};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use tower_http::cors::CorsLayer;

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct TabUpdateEvent {
    timestamp: DateTime<Utc>,
    tab_id: i32,
    url: String,
    title: String,
    type_of_visit: String,
}
#[derive(Deserialize, Serialize, FromRow, Debug)]
struct TabUpdateRow {
    id: i32,
    timestamp: DateTime<Utc>,
    tab_id: i32,
    url: String,
    title: String,
    type_of_visit: String,
}

async fn log_tab_update_event(
    State(pool): State<PgPool>,
    Json(tab_update_event): Json<TabUpdateEvent>,
) -> Result<Json<TabUpdateEvent>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        TabUpdateEvent,
        r#"
        INSERT INTO TAB_UPDATES (timestamp, tab_id, url, title, type_of_visit) 
        VALUES ($1, $2, $3, $4, $5)
        "#,
        tab_update_event.timestamp,
        tab_update_event.tab_id,
        tab_update_event.url,
        tab_update_event.title,
        tab_update_event.type_of_visit
    )
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

async fn return_all_events(
    State(pool): State<PgPool>,
) -> Result<&'static str, (StatusCode, String)> {
    let mut stream = sqlx::query_as!(TabUpdateRow, "SELECT * FROM TAB_UPDATES").fetch(&pool);

    while let Some(tab_update_event) = stream
        .try_next()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        println!("{:?}", tab_update_event);
    }

    Ok("Successfully returned all events!")
}

async fn create_tab_updates_table(
    State(pool): State<PgPool>,
) -> Result<&'static str, (StatusCode, String)> {
    let query = sqlx::query_as!(
        TabUpdateRow,
        r#"
        CREATE TABLE IF NOT EXISTS TAB_UPDATES (
            id SERIAL PRIMARY KEY,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            tab_id int NOT NULL,
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
    let cors = CorsLayer::new()
        .allow_origin(
            "chrome-extension://cdghahhpdoaipdkjjiokakeppeiikobh"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/log_event", post(log_tab_update_event))
        .route("/create_table", post(create_tab_updates_table))
        .route("/return_all_events", get(return_all_events))
        .with_state(pool)
        .layer(cors)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").map_err(|_e| "Couldn't find DATABASE_URL env var")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    let app = create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
