pub mod browse_event;
pub mod cluster;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct PageRow {
    pub id: i32,
    pub url: String,
    pub contents: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, FromRow)]
pub struct PageUrlRow {
    pub url: String,
}

#[derive(FromRow)]
pub struct PreprocessedPageEmbeddingRow {
    pub id: i32,
    pub page_id: i32,
    pub embedding_run: String,
    pub embedding: pgvector::Vector,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
pub struct PageInfoRowWithCluster {
    pub page_url: String,
    pub page_embedding: pgvector::Vector,
    pub cluster_id: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct EventCountBucket {
    pub timestamp_bucket: Option<NaiveDateTime>,
    pub cluster_id: Option<String>,
    pub cluster_name: Option<String>,
    pub clustering_algorithm: Option<String>,
    pub event_count: Option<i64>,
}
