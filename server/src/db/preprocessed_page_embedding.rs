use pgvector::Vector;
use sqlx::{Error, PgPool};

use crate::models::PreprocessedPageEmbeddingRow;

pub async fn insert_preprocessed_page_embedding(
    db: &PgPool,
    page_id: i32,
    embedding_run: &str,
    embedding: &Vector,
) -> Result<PreprocessedPageEmbeddingRow, Error> {
    sqlx::query_as(
        r#"
        INSERT INTO preprocessed_page_embedding (page_id, embedding_run, embedding)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(page_id)
    .bind(embedding_run)
    .bind(embedding)
    .fetch_one(db)
    .await
}
