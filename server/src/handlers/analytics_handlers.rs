use axum::{extract::State, http::StatusCode, Json};
use futures::TryStreamExt;
use sqlx::PgPool;

use crate::db::browse_event::get_all_browse_events;
use crate::models::{BrowseEventRowWithCluster, EventCountBucket};

pub async fn return_all_events(
    State(db): State<PgPool>,
) -> Result<Json<Vec<BrowseEventRowWithCluster>>, (StatusCode, String)> {
    let all_events = get_all_browse_events(&db).await;

    all_events
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_event_buckets(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EventCountBucket>>, (StatusCode, String)> {
    let stream = sqlx::query_as(
        // TODO: add timezone, time range, and interval as query params
        r#"
        WITH timerange_events AS (
            SELECT
                timestamp AT TIME ZONE 'America/New_York' AS local_time, 
                page_cluster_id
            FROM
                browse_event be
                JOIN page_info pi ON be.page_url = pi.page_url
            WHERE timestamp AT TIME ZONE 'America/New_York' >= DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') - INTERVAL '1 day'
                AND timestamp AT TIME ZONE 'America/New_York' < DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') + INTERVAL '1 day'
            ORDER BY timestamp DESC
        )
        SELECT
            DATE_TRUNC('hour', te.local_time) AS timestamp_bucket,
            te.page_cluster_id AS cluster_id,
            c.name::TEXT AS cluster_name,
            COUNT(*) AS event_count
        FROM
            timerange_events te
        LEFT JOIN
            cluster c ON te.page_cluster_id = c.id
        GROUP BY
            DATE_TRUNC('hour', te.local_time),
            te.page_cluster_id,
            c.name
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
