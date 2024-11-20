use askama::Template;
use askama_axum::IntoResponse;
use axum_extra::routing::TypedPath;

use crate::{
    ApplicationMetadata, MANIFEST_DATA, ManifestData, PageDetails, PageMetadata, SentryData,
};
use crate::pages::ApplicationPages;
use crate::utils::languages::SupportedLanguage;

pub struct ThemePageInfo {
    route: ThemeRoute,
    icon: &'static str,
}

impl ThemePageInfo {
    pub fn get_route(&self) -> &ThemeRoute {
        &self.route
    }

    pub fn get_icon(&self) -> &'static str {
        &self.icon
    }
}

impl ApplicationPages {
    pub const THEMES: ThemePageInfo = ThemePageInfo {
        route: ThemeRoute,
        icon: "art",
    };
}

#[derive(TypedPath)]
#[typed_path("/themes")]
pub struct ThemeRoute;

#[derive(Template)]
#[template(path = "application/pages/themes.html")]
struct Themes<'a> {
    page_details: PageDetails<'a>,
}

#[tracing::instrument]
pub async fn render_themes(_: ThemeRoute) -> impl IntoResponse {
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
