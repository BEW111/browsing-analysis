use futures::TryStreamExt;
use pgvector::Vector;
use sqlx::{Error, PgPool};

use crate::models::cluster::{ClusterAssignmentRow, ClusterRow, ClusteringRunRow};

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
    id: &str,
    name: &str,
    clustering_run: &str,
) -> Result<ClusterRow, Error> {
    sqlx::query_as!(
        ClusterRow,
        r#"
        INSERT INTO cluster (id, name, clustering_run)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        id,
        name,
        clustering_run
    )
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
    embedding_run: &str,
    cosine_similarity_threshold: f32,
) -> Result<Option<ClusterAssignmentRow>, Error> {
    sqlx::query_as(
        r#"
        SELECT * FROM cluster_assignment ca
        JOIN preprocessed_page_embedding ppe on ppe.page_id = ca.page_id 
        WHERE ppe.embedding <=> $1 > $2
        AND ppe.embedding_run = $3
        ORDER BY ppe.embedding <=> $1 LIMIT 1
        "#,
    )
    .bind(page_embedding)
    .bind(cosine_similarity_threshold)
    .bind(embedding_run)
    .fetch_optional(db)
    .await
}

pub async fn get_clustering_runs(db: &PgPool) -> Result<Vec<ClusteringRunRow>, Error> {
    let stream = sqlx::query_as!(
        ClusteringRunRow,
        r#"SELECT DISTINCT(clustering_run) FROM cluster"#
    )
    .fetch(db);

    stream.try_collect::<Vec<_>>().await
}
