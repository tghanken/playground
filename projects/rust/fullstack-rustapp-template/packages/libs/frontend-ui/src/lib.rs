pub mod pages;
pub mod router;
mod utils;

use crate::utils::languages::SupportedLanguage;
use once_cell::sync::Lazy;
use vite_manifest_parser::{get_vite_references_str, ViteReferences};

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

static MANIFEST_STR: &str = include_str!(env!("VITE_MANIFEST_PATH"));

static MANIFESTS: Lazy<ViteReferences> =
    Lazy::new(|| get_vite_references_str(MANIFEST_STR).unwrap());
