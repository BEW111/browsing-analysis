use anyhow::Error;
use pgvector::Vector;
use sqlx::PgPool;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::models::{BrowseEventFromChromeExtension, PageInfoRow, PageInfoRowWithCluster};

fn cosine_similarity(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
    // TODO: make this cleaner, error check for vecs of same length

    let dot_product = v1
        .iter()
        .zip(v2.iter())
        .fold(0.0, |acc, (x1, x2)| acc + (x1 * x2));
    let v1_norm = v1.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    let v2_norm = v2.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    return dot_product / (v1_norm * v2_norm);
}

pub async fn assign_cluster(
    pool: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
    page_embedding: &Vector,
) -> Result<String, Error> {
    let nearest_page_info_row: Option<PageInfoRowWithCluster> = sqlx::query_as(
        r#"
    SELECT page_info.page_url as page_url, page_embedding, cluster_id FROM page_info
    JOIN cluster_assignment ca on page_info.page_url = ca.page_url 
    ORDER BY page_embedding <-> $1 LIMIT 1
    "#,
    )
    .bind(&page_embedding)
    .fetch_optional(pool)
    .await?;

    let page_cluster_id = match nearest_page_info_row {
        Some(nearest_page_info_row) => {
            let nearest_page_similarity = cosine_similarity(
                nearest_page_info_row.page_embedding.to_vec(),
                page_embedding.to_vec(),
            );

            match nearest_page_similarity > 0.8 {
                true => nearest_page_info_row.page_cluster_id,
                false => {
                    // TODO: need a better way to come up with cluster ids
                    let mut hasher = DefaultHasher::new();
                    browse_event.page_url.hash(&mut hasher);
                    hasher.finish().to_string()
                }
            }
        }
        None => {
            // TODO: need a better way to come up with cluster ids
            let mut hasher = DefaultHasher::new();
            browse_event.page_url.hash(&mut hasher);
            hasher.finish().to_string()
        }
    };

    Ok(page_cluster_id)
}
