use axum::Router;
use axum_extra::routing::RouterExt;

pub mod dashboard;
pub mod themes;

#[tracing::instrument]
pub fn get_router() -> Router {
    Router::new()
        .typed_get(dashboard::render_dashboard)
        .typed_get(themes::render_themes)
}