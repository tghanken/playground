use askama::Template;
use askama_axum::IntoResponse;

use crate::utils::languages::SupportedLanguage;
use crate::{
    ApplicationMetadata, ManifestData, PageDetails, PageMetadata, SentryData, MANIFEST_DATA,
};

#[derive(Template)]
#[template(path = "pages/themes.html")]
struct Themes<'a> {
    page_details: PageDetails<'a>,
}

#[tracing::instrument]
pub async fn render_themes() -> impl IntoResponse {
    tracing::info!("Rendering themes");
    Themes {
        page_details: PageDetails {
            application_metadata: ApplicationMetadata {
                name: "Pocket Cellar",
            },
            sentry_data: SentryData::default(),
            page_metadata: &PageMetadata {
                title: "Themes",
                lang: SupportedLanguage::English,
            },
            manifest_data: ManifestData {
                stylesheets: Vec::from(&*MANIFEST_DATA.stylesheets),
                scripts: Vec::from(&*MANIFEST_DATA.scripts),
            },
        },
    }
}
