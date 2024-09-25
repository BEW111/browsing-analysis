use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db::{browse_event::get_all_browse_events, page_info::get_pages_in_cluster};
use crate::models::{BrowseEventRowWithCluster, EventCountBucket, PageUrlRow};

pub async fn return_all_events(
    State(db): State<PgPool>,
) -> Result<Json<Vec<BrowseEventRowWithCluster>>, (StatusCode, String)> {
    match get_all_browse_events(&db).await {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_event_buckets(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EventCountBucket>>, (StatusCode, String)> {
    let stream = sqlx::query_as(
        // TODO: add timezone, time range, and interval as query params
        r#"
        WITH timerange_events AS (
            SELECT
                be.timestamp AT TIME ZONE 'America/New_York' AS local_time,
                ca.cluster_id
            FROM
                browse_event be
                JOIN page_info pi ON be.page_url = pi.page_url
                JOIN cluster_assignment ca ON be.page_url = ca.page_url
            WHERE 
                be.timestamp AT TIME ZONE 'America/New_York' >= DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') - INTERVAL '1 day'
                AND be.timestamp AT TIME ZONE 'America/New_York' < DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') + INTERVAL '1 day'
            ORDER BY be.timestamp DESC
        )
        SELECT
            DATE_TRUNC('hour', te.local_time) AS timestamp_bucket,
            te.cluster_id,
            c.name::TEXT AS cluster_name,
            cr.algorithm AS clustering_algorithm,
            COUNT(*) AS event_count
        FROM
            timerange_events te
        LEFT JOIN
            cluster c ON te.cluster_id = c.id
        LEFT JOIN
            clustering_run cr ON c.clustering_run_id = cr.id
        GROUP BY
            DATE_TRUNC('hour', te.local_time),
            te.cluster_id,
            c.name,
            cr.algorithm
        ORDER BY
            timestamp_bucket,
            cluster_id;
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

#[derive(Deserialize)]
pub struct WithClusterId {
    cluster_id: String,
}

pub async fn get_pages(
    State(db): State<PgPool>,
    Query(params): Query<WithClusterId>,
) -> Result<Json<Vec<PageUrlRow>>, (StatusCode, String)> {
    match get_pages_in_cluster(&db, &params.cluster_id).await {
        Ok(pages) => Ok(Json(pages)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
