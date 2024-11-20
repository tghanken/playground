use axum::Router;
use axum_extra::routing::{RouterExt, TypedPath};
use tower_http::services::ServeDir;

use crate::pages::StaticDirectories;

#[tracing::instrument]
fn get_static_router() -> Router {
    let asset_path =
        std::env::var("VITE_ASSETS_PATH").expect("Couldn't find env var VITE_ASSETS_PATH");
    Router::new().nest_service("/", ServeDir::new(asset_path).precompressed_br())
}

#[tracing::instrument]
pub fn get_router() -> Router {
    Router::new()
        .typed_get(crate::pages::dashboard::render_dashboard)
        .typed_get(crate::pages::themes::render_themes)
        .typed_get(healthz)
        .nest(StaticDirectories::VITE_ASSETS, get_static_router())
}

#[derive(TypedPath)]
#[typed_path("/healthz")]
struct HealthzRoute;

#[tracing::instrument(level = "debug")]
async fn healthz(_: HealthzRoute) {
    tracing::debug!("Health check");
}

#[cfg(test)]
mod test {
    use crate::router::get_router;

    #[test]
    fn router_doesnt_panic() {
        let _ = get_router();
    }

    #[test]
    fn router_has_routes() {
        let router = get_router();
        assert!(router.has_routes(), "Router does not have any routes")
    }
}