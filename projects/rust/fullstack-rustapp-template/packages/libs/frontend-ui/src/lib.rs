use std::clone::Clone;
use std::sync::LazyLock;

use vite_manifest_parser::get_vite_references_str;

use crate::utils::languages::SupportedLanguage;
use crate::utils::make_asset_vector;

pub mod pages;
pub mod router;
mod utils;

struct ApplicationMetadata {
    name: &'static str,
}

struct SentryData {
    trace_id: String,
    baggage: String,
}

impl Default for SentryData {
    #[tracing::instrument]
    fn default() -> Self {
        let mut trace_id: String = "".to_string();
        let mut baggage: String = "".to_string();

        if let Some(span) = sentry::configure_scope(|scope| scope.get_span()) {
            for (k, v) in span.iter_headers() {
                match k {
                    "sentry-trace" => {
                        trace_id = v;
                    }
                    "baggage" => {
                        baggage = v;
                    }
                    _ => {}
                }
            }
        }

        SentryData { trace_id, baggage }
    }
}

struct PageMetadata<'a> {
    title: &'a str,
    lang: SupportedLanguage,
}

struct ManifestData {
    stylesheets: Vec<String>,
    scripts: Vec<String>,
}

struct PageDetails<'a> {
    application_metadata: ApplicationMetadata,
    sentry_data: SentryData,
    page_metadata: &'a PageMetadata<'a>,
    manifest_data: ManifestData,
}

static MANIFEST_DATA: LazyLock<ManifestData> = LazyLock::new(|| {
    let manifest_path = std::env::var("VITE_MANIFEST_PATH")
        .expect("VITE_MANIFEST_PATH environment variable must be set");
    let manifest_str = std::fs::read_to_string(manifest_path.to_string()).expect("Cannot read the vite manifest");
    let parsed_manifest = get_vite_references_str(manifest_str.as_str()).expect("Cannot parse the vite manifest");
    let stylesheets = make_asset_vector(parsed_manifest.css_files.clone());
    let scripts = make_asset_vector(parsed_manifest.js_files.clone());
    ManifestData {
        stylesheets,
        scripts
    }
});
