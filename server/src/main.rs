use std::hash::{DefaultHasher, Hash, Hasher};

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
struct BrowseEventFromChromeExtension {
    tab_id: i32,
    timestamp: DateTime<Utc>,
    page_url: String,
    page_title: String,
    page_content: Option<String>,
    event_type: String, // TODO: make this into an enum
}
#[derive(Deserialize, Serialize, FromRow, Debug)]
struct BrowseEventRow {
    id: i32,
    timestamp: DateTime<Utc>,
    tab_id: i32,
    page_url: String,
    page_title: String,
    event_type: String,
}

#[derive(FromRow, Debug)]
struct PageInfoRow {
    page_url: String,
    page_embedding: pgvector::Vector,
    page_cluster_id: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct TabViewBucket {
    timestamp_bucket: Option<String>,
    tab_view_count: Option<i64>,
}

fn should_ignore_event(browse_event: &BrowseEventFromChromeExtension) -> bool {
    // TODO: technically we don't need this since we don't read this urls. but
    //       we should add more urls to this
    if browse_event.page_url == "chrome://extensions" {
        return true;
    }

    false
}

fn cosine_similarity(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
    // TODO: make this cleaner, error check for vecs of same length

    let dot_product = v1
        .iter()
        .zip(v2.iter())
        .fold(0.0, |acc, (x1, x2)| acc + (x1 * x2));
    let v1_norm = v1.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    let v2_norm = v2.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    return dot_product / (v1_norm * v2_norm);
}

async fn update_page_info(
    pool: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<Option<PageInfoRow>, sqlx::Error> {
    let check_row_exists_query_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as num_pages FROM page_info
        WHERE page_url = $1
        LIMIT 1
        "#,
        browse_event.page_url
    )
    .fetch_one(pool)
    .await?;

    if let Some(num_pages) = check_row_exists_query_result.num_pages {
        if num_pages < 1 {
            // TODO: Calculate embedding from page here
            let mut page_embedding_vec = Vec::new();
            for _ in 0..384 {
                page_embedding_vec.push(0.1)
            }
            let page_embedding = pgvector::Vector::from(page_embedding_vec);

            // TODO: Find the closest embedding to this one here, and set the cluster id
            //       appropriately
            let nearest_page_info_row: PageInfoRow = sqlx::query_as(
                r#"
                SELECT * FROM page_info ORDER BY page_embedding <-> $1 LIMIT 1
                "#,
            )
            .bind(&page_embedding)
            .fetch_one(pool)
            .await?;

            let nearest_page_similarity = cosine_similarity(
                nearest_page_info_row.page_embedding.to_vec(),
                page_embedding.to_vec(),
            );

            let page_cluster_id = match nearest_page_similarity > 0.8 {
                true => nearest_page_info_row.page_cluster_id,
                false => {
                    // TODO: need a better way to come up with cluster ids
                    let mut hasher = DefaultHasher::new();
                    browse_event.page_url.hash(&mut hasher);
                    hasher.finish().to_string()
                }
            };

            // Insert the new embedding and cluster id
            let page_info_row: PageInfoRow = sqlx::query_as(
                r#"
                    INSERT INTO page_info (page_url, page_embedding, page_cluster_id)
                    VALUES ($1, $2, $3)
                    RETURNING *
                    "#,
            )
            .bind(&browse_event.page_url)
            .bind(&page_embedding)
            .bind(page_cluster_id)
            .fetch_one(pool)
            .await?;

            return Ok(Some(page_info_row));
        }
    }
    Ok(None)
}

async fn log_browse_event(
    State(pool): State<PgPool>,
    Json(browse_event): Json<BrowseEventFromChromeExtension>,
) -> Result<Json<Option<BrowseEventRow>>, (StatusCode, String)> {
    if should_ignore_event(&browse_event) {
        println!("Ignored event: {:?}", browse_event);
        return Ok(Json(None));
    }

    // TODO: ignore events that are to chrome://extensions, etc.
    println!("Logging event: {:?}", browse_event);

    let insert_tab_update_result = sqlx::query_as!(
        BrowseEventRow,
        r#"
        INSERT INTO browse_event (timestamp, tab_id, page_url, page_title, event_type) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        browse_event.timestamp,
        browse_event.tab_id,
        browse_event.page_url,
        browse_event.page_title,
        browse_event.event_type
    )
    .fetch_one(&pool)
    .await;

    match insert_tab_update_result {
        Ok(uploaded_row) => {
            // TODO: see if there is a nice way to map errors
            let update_page_info_result = update_page_info(&pool, &browse_event).await;

            match update_page_info_result {
                Ok(_) => Ok(Json(Some(uploaded_row))),
                Err(e) => {
                    println!("Failed to upload page info: {:?}", e.to_string());
                    Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                }
            }
        }
        Err(e) => {
            println!("Failed to upload event: {:?}", e.to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }

    // TODO: add page (unique) to a separate table to keep track of
    // different pages
}

async fn return_all_events(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<BrowseEventRow>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(BrowseEventRow, r#"SELECT * FROM browse_event"#).fetch(&pool);
    let collected_events = stream
        .try_collect::<Vec<_>>()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    collected_events
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
            COUNT(event.id) AS tab_view_count
        FROM 
            time_buckets b
        LEFT JOIN 
            browse_event event ON event.timestamp >= b.bucket_start 
                               AND event.timestamp < b.bucket_start + interval '15 minutes'
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
        .route("/log_event", post(log_browse_event))
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
