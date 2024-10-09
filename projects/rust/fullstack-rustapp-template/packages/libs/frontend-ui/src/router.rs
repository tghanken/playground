use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tracing::instrument]
fn get_static_router() -> Router {
    let asset_path =
        std::env::var("VITE_ASSETS_PATH").expect("Couldn't find env var VITE_ASSETS_PATH");
    Router::new().nest_service("/", ServeDir::new(asset_path).precompressed_br())
}

#[tracing::instrument]
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(crate::pages::dashboard::render_dashboard))
        .route("/healthz", get(healthz))
        .nest("/assets", get_static_router())
}

#[tracing::instrument(level = "debug")]
async fn healthz() {
    tracing::debug!("Health check");
}
