use sqlx::{Error, PgPool};

use crate::models::ClusterRow;

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
) -> Result<ClusterRow, Error> {
    sqlx::query_as(
        r#"
            INSERT INTO cluster (id, name)
            VALUES ($1, $2)
            RETURNING *
            "#,
    )
    .bind(cluster_id)
    .bind(cluster_name)
    .fetch_one(db)
    .await
}
