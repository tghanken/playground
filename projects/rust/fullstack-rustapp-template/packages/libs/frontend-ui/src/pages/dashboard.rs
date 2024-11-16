use askama::Template;
use askama_axum::IntoResponse;

use crate::{ApplicationMetadata, MANIFEST_DATA, ManifestData, PageDetails, PageMetadata, SentryData};
use crate::utils::languages::SupportedLanguage;

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
                stylesheets: Vec::from(&*MANIFEST_DATA.stylesheets),
                scripts: Vec::from(&*MANIFEST_DATA.scripts),
            },
        },
    }
}
