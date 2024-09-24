mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

use sqlx::postgres::PgPoolOptions;

use routes::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let app = create_router(db, &config);

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
