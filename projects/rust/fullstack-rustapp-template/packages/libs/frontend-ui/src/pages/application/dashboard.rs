use askama::Template;
use askama_axum::IntoResponse;
use axum_extra::routing::TypedPath;

use crate::{
    ApplicationMetadata, MANIFEST_DATA, ManifestData, PageDetails, PageMetadata, SentryData,
};
use crate::pages::ApplicationPages;
use crate::utils::languages::SupportedLanguage;

pub struct DashboardPageInfo {
    route: DashboardRoute,
    icon: &'static str,
}

impl DashboardPageInfo {
    pub fn get_route(&self) -> &DashboardRoute {
        &self.route
    }

    pub fn get_icon(&self) -> &'static str {
        &self.icon
    }
}

impl ApplicationPages {
    pub const DASHBOARD: DashboardPageInfo = DashboardPageInfo {
        route: DashboardRoute,
        icon: "graph",
    };
}

#[derive(TypedPath)]
#[typed_path("/")]
pub struct DashboardRoute;

#[derive(Template)]
#[template(path = "application/pages/dashboard.html")]
struct Dashboard<'a> {
    page_details: PageDetails<'a>,
}

#[tracing::instrument]
pub async fn render_dashboard(_: DashboardRoute) -> impl IntoResponse {
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
