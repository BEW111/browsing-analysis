use chrono::{DateTime, Utc};
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
