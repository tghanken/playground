use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
pub fn tracing_setup() -> Option<sentry::ClientInitGuard> {
    init_tracing();
    init_sentry()
}

#[tracing::instrument]
fn init_sentry() -> Option<sentry::ClientInitGuard> {
    let sentry_url = match std::env::var("SENTRY_DSN") {
        Ok(url) => url,
        Err(_) => {
            tracing::warn!("No SENTRY_DSN found, disabling sentry");
            return None;
        }
    };
    let environment = std::env::var("SENTRY_ENV").unwrap_or_else(|_| "development".into());
    tracing::info!("Initializing Sentry with environment: {environment}");

    let _guard = sentry::init((
        sentry_url.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(environment.into()),
            // TODO: parse value from env var
            debug: false,
            // TODO: parse value from env var
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));
    tracing::info!("Sentry initialized");
    Some(_guard)
}

#[tracing::instrument]
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                // "frontend=info,tower_http=info,axum::rejection=trace".into()
                "info,axum::rejection=trace".into()
            }),
        )
        .with(sentry::integrations::tracing::layer())
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Tracing initialized");
}
