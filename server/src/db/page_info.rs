use futures::TryStreamExt;
use pgvector::Vector;
use sqlx::{Error, PgPool};

use crate::models::{PageInfoRow, PageInfoRowWithCluster, PageUrlRow};

pub async fn check_page_info_exists(db: &PgPool, page_url: &String) -> Result<bool, Error> {
    let check_row_exists_query_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as num_pages FROM page_info
        WHERE page_url = $1
        LIMIT 1
        "#,
        page_url
    )
    .fetch_one(db)
    .await?;

    Ok(check_row_exists_query_result.num_pages.unwrap_or(0) >= 1)
}

pub async fn insert_page_info(
    db: &PgPool,
    page_url: &String,
    page_embedding: &Vector,
) -> Result<PageInfoRow, Error> {
    sqlx::query_as(
        r#"
            INSERT INTO page_info (page_url, page_embedding)
            VALUES ($1, $2)
            RETURNING *
            "#,
    )
    .bind(&page_url)
    .bind(&page_embedding)
    .fetch_one(db)
    .await
}

pub async fn get_nearest_page_info(
    db: &PgPool,
    page_embedding: &Vector,
) -> Result<Option<PageInfoRowWithCluster>, Error> {
    sqlx::query_as(
        r#"
        SELECT page_info.page_url as page_url, page_embedding, cluster_id FROM page_info
        JOIN cluster_assignment ca on page_info.page_url = ca.page_url 
        ORDER BY page_embedding <-> $1 LIMIT 1
        "#,
    )
    .bind(&page_embedding)
    .fetch_optional(db)
    .await
}

pub async fn get_pages_in_cluster(
    db: &PgPool,
    cluster_id: &String,
) -> Result<Vec<PageUrlRow>, Error> {
    let stream = sqlx::query_as!(
        PageUrlRow,
        r#"
        SELECT page_info.page_url FROM page_info
        JOIN cluster_assignment ON page_info.page_url = cluster_assignment.page_url
        WHERE cluster_id = $1
        "#,
        cluster_id
    )
    .fetch(db);

    stream.try_collect::<Vec<_>>().await
}
