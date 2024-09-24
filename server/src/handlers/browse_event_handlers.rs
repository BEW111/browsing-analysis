use anyhow::Error;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::db::{
    browse_event::insert_browse_event,
    cluster::{check_cluster_exists, insert_cluster, insert_cluster_assignment},
    page_info::{check_page_info_exists, insert_page_info},
};
use crate::models::{BrowseEventFromChromeExtension, BrowseEventRow, ClusterRow, PageInfoRow};
use crate::services::clustering::assign_cluster;
use crate::services::page_processing::{
    extract_keywords, get_embedding_from_text, get_markdown_from_html,
};

pub async fn log_browse_event(
    State(db): State<PgPool>,
    Json(browse_event): Json<BrowseEventFromChromeExtension>,
) -> Result<Json<Option<BrowseEventRow>>, (StatusCode, String)> {
    if should_ignore_event(&browse_event) {
        println!("Ignored event: {:?}", browse_event.page_url);
        return Ok(Json(None));
    }

    println!("Logging event: {:?}", browse_event.page_url);

    match insert_browse_event(&db, &browse_event).await {
        Ok(uploaded_row) => process_browse_event_page(&db, &browse_event)
            .await
            .map(|_| Json(Some(uploaded_row)))
            .map_err(|e| {
                eprintln!("Failed to upload page info: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }),
        Err(e) => {
            eprintln!("Failed to upload event: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

fn should_ignore_event(browse_event: &BrowseEventFromChromeExtension) -> bool {
    if browse_event.page_url.contains("localhost") {
        return true;
    }

    false
}

async fn process_browse_event_page(
    db: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<Option<PageInfoRow>, Error> {
    let page_info_exists = check_page_info_exists(db, &browse_event.page_url).await?;

    if !page_info_exists {
        if let Some(page_content) = &browse_event.page_content {
            let page_markdown = get_markdown_from_html(page_content)?;
            let page_embedding = get_embedding_from_text(&page_markdown)?;
            let page_cluster_id = assign_cluster(db, browse_event, &page_embedding).await?;

            let page_info_row =
                insert_page_info(db, &browse_event.page_url, &page_embedding).await?;
            update_cluster_info(&page_markdown, &page_cluster_id, db).await?;
            insert_cluster_assignment(db, &browse_event.page_url, &page_cluster_id).await?;

            return Ok(Some(page_info_row));
        }
    }

    Ok(None)
}

// TODO: make this into just one implementation of a clustering algo
async fn update_cluster_info(
    page_markdown: &String,
    cluster_id: &String,
    db: &PgPool,
) -> Result<Option<ClusterRow>, Error> {
    let cluster_exists = check_cluster_exists(db, cluster_id).await?;

    if !cluster_exists {
        // TODO: make `num_keywords` into a global param/const
        let num_keywords = 5;
        let clustering_run_id = 1;
        let cluster_keywords = extract_keywords(page_markdown, num_keywords);
        let cluster_name = cluster_keywords.join(" ");
        let cluster_row: ClusterRow =
            insert_cluster(db, &cluster_id, &cluster_name, clustering_run_id).await?;

        return Ok(Some(cluster_row));
    }

    Ok(None)
}
