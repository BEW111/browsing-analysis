use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct ClusterRow {
    pub id: String,
    pub name: String,
    pub clustering_run: String,
}

#[derive(FromRow)]
pub struct ClusterAssignmentRow {
    pub id: i32,
    pub page_id: i32,
    pub cluster_id: String,
}

#[derive(FromRow, Serialize)]
pub struct ClusteringRunRow {
    pub clustering_run: String,
}
