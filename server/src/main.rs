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
struct TabUpdateEventFromChromeExtension {
    timestamp: DateTime<Utc>,
    tab_id: i32,
    url: String,
    title: String,
    type_of_visit: String, // TODO: make this into an enum
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

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct TabViewBucket {
    timestamp_bucket: Option<String>,
    tab_view_count: Option<i64>,
}

async fn log_tab_update_event(
    State(pool): State<PgPool>,
    Json(tab_update_event): Json<TabUpdateEventFromChromeExtension>,
) -> Result<Json<TabUpdateRow>, (StatusCode, String)> {
    // TODO: ignore events that are to chrome://extensions, etc.
    println!("Received event: {:?}", tab_update_event);

    let insert_tab_update_result = sqlx::query_as!(
        TabUpdateRow,
        r#"
        INSERT INTO TAB_UPDATES (timestamp, tab_id, url, title, type_of_visit) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        tab_update_event.timestamp,
        tab_update_event.tab_id,
        tab_update_event.url,
        tab_update_event.title,
        tab_update_event.type_of_visit
    )
    .fetch_one(&pool)
    .await;

    match insert_tab_update_result {
        Ok(uploaded_row) => Ok(Json(uploaded_row)),
        Err(e) => {
            println!("Failed to upload event");
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }

    // TODO: add page (unique) to a separate table to keep track of
    // different pages
}

async fn return_all_events(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<TabUpdateRow>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(TabUpdateRow, r#"SELECT * FROM TAB_UPDATES"#).fetch(&pool);
    let collected_events = stream
        .try_collect::<Vec<_>>()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    collected_events
}

async fn create_tab_updates_table(
    State(pool): State<PgPool>,
) -> Result<&'static str, (StatusCode, String)> {
    let query = sqlx::query_as!(
        TabUpdateRow,
        "
        CREATE TABLE IF NOT EXISTS TAB_UPDATES (
            id SERIAL PRIMARY KEY,
            timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
            tab_id int NOT NULL,
            url TEXT NOT NULL,
            title TEXT NOT NULL,
            type_of_visit TEXT NOT NULL
        )
        ",
    );

    match query.execute(&pool).await {
        Ok(_) => Ok("Table 'events' created successfully"),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_tab_view_buckets(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<TabViewBucket>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(
        TabViewBucket,
        // TODO: add time range and interval as query params
        r#"
        WITH time_range AS (
            SELECT 
                date_trunc('hour', NOW() - INTERVAL '24 hours') AS start_time,
                date_trunc('hour', NOW()) AS end_time
        ),
        time_buckets AS (
            SELECT generate_series(
                start_time,
                end_time,
                interval '15 minutes'
            ) AS bucket_start
            FROM time_range
        )
        SELECT 
            to_char(b.bucket_start, 'YYYY-MM-DD HH24:MI') AS timestamp_bucket,
            COUNT(tu.id) AS tab_view_count
        FROM 
            time_buckets b
        LEFT JOIN 
            TAB_UPDATES tu ON tu.timestamp >= b.bucket_start 
                           AND tu.timestamp < b.bucket_start + interval '15 minutes'
        GROUP BY 
            b.bucket_start
        ORDER BY 
            b.bucket_start;
    "#
    )
    .fetch(&pool);

    let collected_events = stream
        .try_collect::<Vec<_>>()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    collected_events
}

pub fn create_router(pool: PgPool) -> Router {
    let origins = [
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        "chrome-extension://cdghahhpdoaipdkjjiokakeppeiikobh"
            .parse::<HeaderValue>()
            .unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/log_event", post(log_tab_update_event))
        .route("/create_table", post(create_tab_updates_table))
        .route("/return_all_events", get(return_all_events))
        .route("/get_tab_view_buckets", get(get_tab_view_buckets))
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
