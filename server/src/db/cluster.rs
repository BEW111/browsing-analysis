use futures::TryStreamExt;
use pgvector::Vector;
use sqlx::{Error, PgPool};

use crate::models::cluster::{ClusterAssignmentRow, ClusterRow};

pub async fn check_cluster_exists(db: &PgPool, cluster_id: &str) -> Result<bool, Error> {
    let check_row_exists_query_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as num_clusters FROM cluster
        WHERE id = $1
        LIMIT 1
        "#,
        cluster_id
    )
    .fetch_one(db)
    .await?;

    Ok(check_row_exists_query_result.num_clusters.unwrap_or(0) >= 1)
}

pub async fn insert_cluster(
    db: &PgPool,
    cluster_id: &str,
    cluster_name: &str,
    clustering_run_id: i32,
) -> Result<ClusterRow, Error> {
    sqlx::query_as(
        r#"
            INSERT INTO cluster (id, name, clustering_run_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
    )
    .bind(cluster_id)
    .bind(cluster_name)
    .bind(clustering_run_id)
    .fetch_one(db)
    .await
}

pub async fn insert_cluster_assignment(
    db: &PgPool,
    page_id: i32,
    cluster_id: &str,
) -> Result<ClusterAssignmentRow, Error> {
    sqlx::query_as!(
        ClusterAssignmentRow,
        r#"
        INSERT INTO cluster_assignment (cluster_id, page_id)
        VALUES ($1, $2)
        RETURNING *
        "#,
        cluster_id,
        page_id
    )
    .fetch_one(db)
    .await
}

pub async fn get_all_clusters(db: &PgPool) -> Result<Vec<ClusterRow>, Error> {
    let stream = sqlx::query_as!(
        ClusterRow,
        r#"
        SELECT * FROM cluster
        "#
    )
    .fetch(db);

    stream.try_collect::<Vec<_>>().await
}

pub async fn get_nearest_cluster_above_similarity_threshold(
    db: &PgPool,
    page_embedding: &Vector,
    cosine_similarity_threshold: f32,
) -> Result<Option<ClusterAssignmentRow>, Error> {
    sqlx::query_as(
        r#"
        SELECT * FROM cluster_assignment ca
        JOIN preprocessed_page_embedding ppe on ppe.page_id = ca.page_id 
        WHERE ppe.embedding <=> $1 > $2
        ORDER BY ppe.embedding <=> $1 LIMIT 1
        "#,
    )
    .bind(page_embedding)
    .bind(cosine_similarity_threshold)
    .fetch_optional(db)
    .await
}
