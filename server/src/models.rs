use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct BrowseEventFromChromeExtension {
    pub tab_id: i32,
    pub timestamp: DateTime<Utc>,
    pub page_url: String,
    pub page_title: String,
    pub page_content: Option<String>,
    pub event_type: String, // TODO: make this into an enum
}
#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct BrowseEventRow {
    pub id: i32,
    pub timestamp: DateTime<Utc>,
    pub tab_id: i32,
    pub page_url: String,
    pub page_title: String,
    pub event_type: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct BrowseEventRowWithCluster {
    pub id: i32,
    pub timestamp: DateTime<Utc>,
    pub tab_id: i32,
    pub page_url: String,
    pub page_title: String,
    pub page_cluster_id: Option<String>,
    pub event_type: String,
}

#[derive(FromRow)]
pub struct PageInfoRow {
    pub page_url: String,
    pub page_embedding: pgvector::Vector,
    pub page_cluster_id: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct EventCountBucket {
    pub timestamp_bucket: Option<NaiveDateTime>,
    pub cluster_id: Option<String>,
    pub cluster_name: Option<String>,
    pub event_count: Option<i64>,
}

#[derive(FromRow)]
pub struct ClusterRow {
    pub id: String,
    pub name: String,
}
