use pgvector::Vector;
use sqlx::{Error, PgPool};

use crate::models::PageInfoRow;

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
    page_cluster_id: &String,
) -> Result<PageInfoRow, Error> {
    sqlx::query_as(
        r#"
            INSERT INTO page_info (page_url, page_embedding, page_cluster_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
    )
    .bind(&page_url)
    .bind(&page_embedding)
    .bind(&page_cluster_id)
    .fetch_one(db)
    .await
}
