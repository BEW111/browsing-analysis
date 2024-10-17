use futures::TryStreamExt;
use sqlx::{Error, PgPool};

use crate::models::{PageRow, PageUrlRow};

pub async fn get_page_from_url(db: &PgPool, page_url: &String) -> Result<Option<PageRow>, Error> {
    sqlx::query_as!(
        PageRow,
        r#"
        SELECT * FROM page
        WHERE url = $1
        "#,
        page_url
    )
    .fetch_optional(db)
    .await
}

pub async fn insert_page(db: &PgPool, url: &String, contents: &String) -> Result<PageRow, Error> {
    sqlx::query_as!(
        PageRow,
        r#"
        INSERT INTO page (url, contents)
        VALUES ($1, $2)
        RETURNING *
        "#,
        url,
        contents
    )
    .fetch_one(db)
    .await
}

pub async fn update_page(db: &PgPool, url: &str, new_contents: &str) -> Result<PageRow, Error> {
    sqlx::query_as!(
        PageRow,
        r#"
        UPDATE page
        SET contents = $1
        WHERE url = $2
        RETURNING *
        "#,
        new_contents,
        url
    )
    .fetch_one(db)
    .await
}

pub async fn get_pages_in_cluster(
    db: &PgPool,
    cluster_id: &String,
) -> Result<Vec<PageUrlRow>, Error> {
    let stream = sqlx::query_as!(
        PageUrlRow,
        r#"
        SELECT page.url FROM page
        JOIN cluster_assignment ca ON page.id = ca.page_id
        WHERE cluster_id = $1
        "#,
        cluster_id
    )
    .fetch(db);

    stream.try_collect::<Vec<_>>().await
}
