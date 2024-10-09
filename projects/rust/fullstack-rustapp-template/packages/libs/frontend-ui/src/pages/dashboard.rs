use crate::utils::languages::SupportedLanguage;
use crate::utils::make_asset_vector;
use crate::{ApplicationMetadata, ManifestData, PageDetails, PageMetadata, SentryData, MANIFESTS};
use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
struct Dashboard<'a> {
    page_details: PageDetails<'a>,
}

#[tracing::instrument]
pub async fn render_dashboard() -> impl IntoResponse {
    tracing::info!("Rendering dashboard");
    Dashboard {
        page_details: PageDetails {
            application_metadata: ApplicationMetadata {
                name: "Pocket Cellar",
            },
            sentry_data: SentryData::default(),
            page_metadata: &PageMetadata {
                title: "Dashboard",
                lang: SupportedLanguage::English,
            },
            manifest_data: ManifestData {
                stylesheets: make_asset_vector(MANIFESTS.css_files.clone()),
                scripts: make_asset_vector(MANIFESTS.js_files.clone()),
            },
        },
    }
}
