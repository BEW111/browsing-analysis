use futures::TryStreamExt;
use sqlx::{Error, PgPool};

use crate::models::{BrowseEventFromChromeExtension, BrowseEventRow, BrowseEventRowWithCluster};

pub async fn insert_browse_event(
    db: &PgPool,
    browse_event: &BrowseEventFromChromeExtension,
) -> Result<BrowseEventRow, Error> {
    sqlx::query_as!(
        BrowseEventRow,
        r#"
        INSERT INTO browse_event (timestamp, tab_id, page_url, page_title, event_type) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        browse_event.timestamp,
        browse_event.tab_id,
        browse_event.page_url,
        browse_event.page_title,
        browse_event.event_type
    )
    .fetch_one(db)
    .await
}

pub async fn get_all_browse_events(db: &PgPool) -> Result<Vec<BrowseEventRowWithCluster>, Error> {
    let stream = sqlx::query_as!(
        BrowseEventRowWithCluster,
        r#"
        SELECT browse_event.id as id, timestamp, tab_id, browse_event.page_url as page_url, page_title, ca.cluster_id as page_cluster_id, event_type FROM browse_event
        LEFT JOIN page_info ON browse_event.page_url = page_info.page_url
        LEFT JOIN cluster_assignment ca ON page_info.page_url = ca.page_url
        "#
    )
    .fetch(db);

    stream.try_collect::<Vec<_>>().await
}
