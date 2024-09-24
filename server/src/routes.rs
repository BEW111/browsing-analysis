use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;
use crate::handlers::analytics_handlers::{get_event_buckets, return_all_events};
use crate::handlers::browse_event_handlers::log_browse_event;

pub fn create_router(db: PgPool, config: &Config) -> Router {
    let cors = create_cors_layer(config);

    Router::new()
        .route("/log_event", post(log_browse_event))
        .route("/return_all_events", get(return_all_events))
        .route("/get_event_buckets", get(get_event_buckets))
        .with_state(db)
        .layer(cors)
}

fn create_cors_layer(config: &Config) -> CorsLayer {
    let origins = [
        config.frontend_url.parse::<HeaderValue>().unwrap(),
        config.extension_url.parse::<HeaderValue>().unwrap(),
    ];

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
