use anyhow::Error;
use pgvector::Vector;
use sqlx::PgPool;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    db::cluster::get_nearest_cluster_above_similarity_threshold,
    models::{browse_event::BrowseEventFromChromeExtension, cluster::ClusterAssignmentRow},
};

fn cosine_similarity(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
    // TODO: make this cleaner, error check for vecs of same length

    let dot_product = v1
        .iter()
        .zip(v2.iter())
        .fold(0.0, |acc, (x1, x2)| acc + (x1 * x2));
    let v1_norm = v1.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    let v2_norm = v2.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
    dot_product / (v1_norm * v2_norm)
}

pub async fn assign_page_to_cluster_id(
    db: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
    page_embedding: &Vector,
) -> Result<String, Error> {
    let close_enough_cluster_distance = 0.8;

    let cluster_assignment_row: Option<ClusterAssignmentRow> =
        get_nearest_cluster_above_similarity_threshold(
            db,
            page_embedding,
            close_enough_cluster_distance,
        )
        .await?;

    let page_cluster_id = match cluster_assignment_row {
        Some(cluster_assignment_row) => cluster_assignment_row.cluster_id,
        None => {
            // TODO: need a better way to come up with cluster ids
            let mut hasher = DefaultHasher::new();
            browse_event.page_url.hash(&mut hasher);
            hasher.finish().to_string()
        }
    };

    Ok(page_cluster_id)
}
