use anyhow::Error;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::db::{
    browse_event::insert_browse_event,
    cluster::{check_cluster_exists, insert_cluster},
    page_info::{check_page_info_exists, insert_page_info},
};
use crate::models::{BrowseEventFromChromeExtension, BrowseEventRow, ClusterRow, PageInfoRow};
use crate::services::clustering::assign_cluster;
use crate::services::page_processing::{
    extract_keywords, generate_markdown, generate_pgvector_embedding,
};

fn should_ignore_event(browse_event: &BrowseEventFromChromeExtension) -> bool {
    // TODO: technically we don't need this since we don't read this url. but
    //       we should add more urls to this
    if browse_event.page_url == "chrome://extensions" {
        return true;
    } else if browse_event.page_url.contains("localhost") {
        return true;
    }

    false
}

async fn update_cluster_info(
    page_markdown: &String,
    cluster_id: &String,
    db: &PgPool,
) -> Result<Option<ClusterRow>, Error> {
    let cluster_exists = check_cluster_exists(db, cluster_id).await?;

    if !cluster_exists {
        // TODO: make `num_keywords` into a param/const
        let num_keywords = 5;
        let cluster_keywords = extract_keywords(page_markdown, num_keywords);
        let cluster_name = cluster_keywords.join(" ");
        let cluster_row: ClusterRow = insert_cluster(db, &cluster_id, &cluster_name).await?;

        return Ok(Some(cluster_row));
    }

    Ok(None)
}

async fn update_page_info(
    db: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<Option<PageInfoRow>, Error> {
    let page_info_exists = check_page_info_exists(db, &browse_event.page_url).await?;

    if !page_info_exists {
        if let Some(page_content) = &browse_event.page_content {
            let page_markdown = generate_markdown(page_content)?;
            let page_embedding = generate_pgvector_embedding(&page_markdown)?;
            let page_cluster_id = assign_cluster(db, browse_event, &page_embedding).await?;

            // Insert the new embedding and cluster id
            println!("Inserting embedding into db...");
            let page_info_row: PageInfoRow = insert_page_info(
                db,
                &browse_event.page_url,
                &page_embedding,
                &page_cluster_id,
            )
            .await?;
            println!("Finished inserting embedding!");

            update_cluster_info(&page_markdown, &page_cluster_id, db).await?;

            return Ok(Some(page_info_row));
        }
    }

    Ok(None)
}

pub async fn log_browse_event(
    State(db): State<PgPool>,
    Json(browse_event): Json<BrowseEventFromChromeExtension>,
) -> Result<Json<Option<BrowseEventRow>>, (StatusCode, String)> {
    if should_ignore_event(&browse_event) {
        println!("Ignored event: {:?}", browse_event.page_url);
        return Ok(Json(None));
    }

    // TODO: ignore events that are to chrome://extensions, etc.
    println!("Logging event: {:?}", browse_event.page_url);

    let insert_tab_update_result = insert_browse_event(&db, &browse_event).await;

    match insert_tab_update_result {
        Ok(uploaded_row) => {
            // TODO: see if there is a nice way to map errors
            let update_page_info_result = update_page_info(&db, &browse_event).await;

            match update_page_info_result {
                Ok(_) => Ok(Json(Some(uploaded_row))),
                Err(e) => {
                    println!("Failed to upload page info: {:?}", e.to_string());
                    Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                }
            }
        }
        Err(e) => {
            println!("Failed to upload event: {:?}", e.to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
