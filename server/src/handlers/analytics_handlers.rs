use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::PgPool;

use crate::db;
use crate::{
    db::{
        browse_event::get_all_browse_events, cluster::get_all_clusters, page::get_pages_in_cluster,
    },
    models::{
        browse_event::BrowseEventRowWithCluster,
        cluster::{ClusterRow, ClusteringRunRow},
        EventCountBucket, PageUrlRow,
    },
};

pub async fn return_all_events(
    State(db): State<PgPool>,
) -> Result<Json<Vec<BrowseEventRowWithCluster>>, (StatusCode, String)> {
    match get_all_browse_events(&db).await {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct WithClusteringRun {
    clustering_run: String,
}

pub async fn get_event_buckets(
    State(pool): State<PgPool>,
    Query(params): Query<WithClusteringRun>,
) -> Result<Json<Vec<EventCountBucket>>, (StatusCode, String)> {
    let stream = sqlx::query_as!(
        // TODO: add timezone, time range, and interval as query params
        EventCountBucket,
        r#"
        WITH timerange_events AS (
            SELECT
                be.timestamp AT TIME ZONE 'America/New_York' AS local_time,
                ca.cluster_id
            FROM
                browse_event be
                JOIN page ON be.page_url = page.url
                JOIN cluster_assignment ca ON page.id = ca.page_id
            WHERE 
                be.timestamp AT TIME ZONE 'America/New_York' >= DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') - INTERVAL '1 day'
                AND be.timestamp AT TIME ZONE 'America/New_York' < DATE_TRUNC('day', CURRENT_TIMESTAMP AT TIME ZONE 'America/New_York') + INTERVAL '1 day'
            ORDER BY be.timestamp DESC
        )
        SELECT
            DATE_TRUNC('hour', te.local_time) AS timestamp_bucket,
            te.cluster_id,
            c.name::TEXT AS cluster_name,
            COUNT(*) AS event_count
        FROM
            timerange_events te
        LEFT JOIN
            cluster c ON te.cluster_id = c.id
        WHERE c.clustering_run = $1
        GROUP BY
            DATE_TRUNC('hour', te.local_time),
            te.cluster_id,
            c.name
        ORDER BY
            timestamp_bucket,
            cluster_id;
        "#,
        &params.clustering_run
    )
    .fetch(&pool);

    stream
        .try_collect::<Vec<_>>()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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

pub async fn get_clusters(
    State(db): State<PgPool>,
) -> Result<Json<Vec<ClusterRow>>, (StatusCode, String)> {
    match get_all_clusters(&db).await {
        Ok(clusters) => Ok(Json(clusters)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_clustering_runs(
    State(db): State<PgPool>,
) -> Result<Json<Vec<ClusteringRunRow>>, (StatusCode, String)> {
    match db::cluster::get_clustering_runs(&db).await {
        Ok(clustering_runs) => Ok(Json(clustering_runs)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
