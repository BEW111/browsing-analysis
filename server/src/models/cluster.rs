use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct ClusterRow {
    pub id: String,
    pub name: String,
    pub clustering_run_id: i32,
}

#[derive(FromRow)]
pub struct ClusterAssignmentRow {
    pub id: i32,
    pub page_id: i32,
    pub cluster_id: String,
}
