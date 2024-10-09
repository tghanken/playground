use axum::{extract::MatchedPath, http::Request, Router};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tracing::instrument]
pub fn add_middlewares(router: Router) -> Router {
    let tracing_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        // Log the matched route's path (with placeholders not filled in).
        // Use request.uri() or OriginalUri if you want the real path.
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);

        tracing::info_span!(
            "http_request",
            method = ?request.method(),
            matched_path,
            raw_path = ?request.uri(),
        )
    });

    let services = ServiceBuilder::new()
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction())
        .layer(tracing_layer)
        .layer(tower_http::compression::CompressionLayer::new());

    #[cfg(feature = "tower-livereload")]
    tracing::info!("live reload enabled");
    #[cfg(feature = "tower-livereload")]
    let services = services.layer(tower_livereload::LiveReloadLayer::new());

    router.layer(services)
}
