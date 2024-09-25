use futures::TryStreamExt;
use sqlx::{Error, PgPool};

use crate::models::{ClusterAssignmentRow, ClusterRow};

pub async fn check_cluster_exists(db: &PgPool, cluster_id: &String) -> Result<bool, Error> {
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
    cluster_id: &String,
    cluster_name: &String,
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
    page_url: &String,
    cluster_id: &String,
) -> Result<ClusterAssignmentRow, Error> {
    sqlx::query_as(
        r#"
            INSERT INTO cluster_assignment (page_url, cluster_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
    )
    .bind(page_url)
    .bind(cluster_id)
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
