mod cluster;
mod config;
mod embedding;
mod models;

use cluster::assign_cluster;
use embedding::generate_pgvector_embedding;
use models::{
    BrowseEventFromChromeExtension, BrowseEventRow, BrowseEventRowWithCluster, EventCountBucket,
    PageInfoRow,
};

use anyhow::Error;
use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, PgPool};

use tower_http::cors::CorsLayer;

fn should_ignore_event(browse_event: &BrowseEventFromChromeExtension) -> bool {
    // TODO: technically we don't need this since we don't read this url. but
    //       we should add more urls to this
    if browse_event.page_url == "chrome://extensions" {
        return true;
    }

    false
}

// TODO: move all hard-coded constants in here outside
async fn update_page_info(
    pool: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<Option<PageInfoRow>, Error> {
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
            if let Some(page_content) = &browse_event.page_content {
                // Generate embedding
                let page_embedding = generate_pgvector_embedding(page_content)?;

                // Assign page to cluster
                let page_cluster_id = assign_cluster(pool, browse_event, &page_embedding).await?;

                // Insert the new embedding and cluster id
                println!("Inserting embedding into db");
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
                println!("Finished inserting embedding!");

                return Ok(Some(page_info_row));
            }
        } else {
            println!("Page has already been logged")
        }
    }
    Ok(None)
}

async fn log_browse_event(
    State(pool): State<PgPool>,
    Json(browse_event): Json<BrowseEventFromChromeExtension>,
) -> Result<Json<Option<BrowseEventRow>>, (StatusCode, String)> {
    if should_ignore_event(&browse_event) {
        println!("Ignored event: {:?}", browse_event.page_url);
        return Ok(Json(None));
    }

    // TODO: ignore events that are to chrome://extensions, etc.
    println!("Logging event: {:?}", browse_event.page_url);

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
}

async fn return_all_events(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<BrowseEventRowWithCluster>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(
        BrowseEventRowWithCluster,
        r#"
        SELECT id, timestamp, tab_id, browse_event.page_url as page_url, page_title, page_cluster_id, event_type FROM browse_event
        LEFT JOIN page_info ON browse_event.page_url = page_info.page_url
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

async fn get_event_buckets(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EventCountBucket>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(
        EventCountBucket,
        // TODO: add timezone, time range, and interval as query params
        r#"
        WITH timerange_events AS (
            SELECT
                timestamp AT TIME ZONE 'America/New_York' AS local_time, page_cluster_id
            FROM
                browse_event be
                JOIN page_info pi ON be.page_url = pi.page_url
            WHERE timestamp AT TIME ZONE 'America/New_York' >= DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') - INTERVAL '1 day'
                AND timestamp AT TIME ZONE 'America/New_York' < DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') + INTERVAL '1 day'
            ORDER BY timestamp DESC
        )
        SELECT
            DATE_TRUNC('hour', local_time) AS timestamp_bucket,
            page_cluster_id AS cluster_id,
            COUNT(*) AS event_count
        FROM
            timerange_events
        GROUP BY
            DATE_TRUNC('hour', local_time),
            page_cluster_id
        ORDER BY
            timestamp_bucket,
            cluster_id
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

pub fn create_router(db: PgPool, config: &config::Config) -> Router {
    let origins = [
        config.frontend_url.parse::<HeaderValue>().unwrap(),
        config.extension_url.parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/log_event", post(log_browse_event))
        .route("/return_all_events", get(return_all_events))
        .route("/get_event_buckets", get(get_event_buckets))
        .with_state(db)
        .layer(cors)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let app = create_router(db, &config);

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
