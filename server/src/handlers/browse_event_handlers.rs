use anyhow::Error;
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::{
    db::{
        browse_event::insert_browse_event,
        cluster::{check_cluster_exists, insert_cluster, insert_cluster_assignment},
        page::{get_page_from_url, insert_page, update_page},
        preprocessed_page_embedding::insert_preprocessed_page_embedding,
    },
    models::{
        browse_event::{BrowseEventFromChromeExtension, BrowseEventRow},
        cluster::ClusterRow,
        PageRow,
    },
    services::{
        clustering::assign_page_to_cluster_id,
        preprocessing::pipelines,
        utils::{extract_keywords, html_to_markdown},
    },
};

#[debug_handler]
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
                eprintln!("Failed to process/upload page info: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }),
        Err(e) => {
            eprintln!("Failed to upload event: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

fn should_ignore_event(browse_event: &BrowseEventFromChromeExtension) -> bool {
    if browse_event.page_url.starts_with("http://localhost") {
        return true;
    }

    if browse_event.page_url.starts_with("https://mail.google.com") {
        return true;
    }

    false
}

async fn process_browse_event_page(
    db: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<Option<PageRow>, Error> {
    // If the page exists already, then we don't apply online clustering strategies to it,
    // even if the strategies are new. Batch strategies will always run on new pages.
    let url = &browse_event.page_url;

    let (page_row_missing, page_contents_missing) = match get_page_from_url(db, url).await? {
        Some(page_row) => (false, page_row.contents.is_none()),
        None => (true, true),
    };

    if page_contents_missing {
        if let Some(page_content) = &browse_event.page_content {
            let page_row = match page_row_missing {
                true => insert_page(db, url, page_content).await?,
                false => update_page(db, url, page_content).await?,
            };

            // TODO: get multiple different pipelines and run them all
            let preprocessing_pipelines = pipelines::get_all_preprocessing_pipelines()?;
            for preprocessing_pipeline in preprocessing_pipelines {
                let embedding = preprocessing_pipeline.run(page_content)?;
                insert_preprocessed_page_embedding(
                    db,
                    page_row.id,
                    preprocessing_pipeline.name,
                    &embedding,
                )
                .await?;

                let page_cluster_id =
                    assign_page_to_cluster_id(db, browse_event, &embedding).await?;

                // TODO: could have slightly better naming here to indicate that "create" refers to adding to the db?
                create_cluster_if_not_exists(db, page_content, &page_cluster_id, 1).await?;
                insert_cluster_assignment(db, page_row.id, &page_cluster_id).await?;
            }

            return Ok(Some(page_row));
        }
    }

    Ok(None)
}

// TODO: make this into just one implementation of a clustering algo
async fn create_cluster_if_not_exists(
    db: &PgPool,
    page_content: &str,
    cluster_id: &str,
    clustering_run_id: i32,
) -> Result<Option<ClusterRow>, Error> {
    let cluster_exists = check_cluster_exists(db, cluster_id).await?;

    if !cluster_exists {
        // TODO: make `num_keywords` into a global param/const
        // Also rework this whole thing later
        let num_keywords = 5;
        let page_markdown = html_to_markdown(page_content)?;
        let cluster_keywords = extract_keywords(&page_markdown, num_keywords);
        let cluster_name = cluster_keywords.join(" ");
        let cluster_row: ClusterRow =
            insert_cluster(db, cluster_id, &cluster_name, clustering_run_id).await?;

        return Ok(Some(cluster_row));
    }

    Ok(None)
}
