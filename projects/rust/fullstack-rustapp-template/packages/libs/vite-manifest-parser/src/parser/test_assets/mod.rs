use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::parser::{InternalManifest, InternalManifestDetails};

pub static SIMPLE_MANIFEST_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("src/parser/test_assets/assets/simple_manifest.json"));
pub const SIMPLE_MANIFEST_JSON_PRETTY: &str = include_str!("./assets/simple_manifest.json");
pub static SIMPLE_MANIFEST_CSS: LazyLock<HashSet<String>> =
    LazyLock::new(|| HashSet::from_iter(vec!["shared-ChJ_j-JJ.css".to_string()]));
pub static SIMPLE_MANIFEST_JS: LazyLock<HashSet<String>> = LazyLock::new(|| HashSet::from_iter(vec![]));
pub static SIMPLE_MANIFEST: LazyLock<InternalManifest> = LazyLock::new(|| {
    let mut bundles = HashMap::new();

    bundles.insert(
        "_shared-v2JlcC3m.js".to_string(),
        InternalManifestDetails {
            css: None,
            file: "assets/shared-ChJ_j-JJ.css".to_string(),
            name: None,
            src: Some("_shared-v2JlcC3m.js".to_string()),
            is_dynamic_entry: None,
            is_entry: None,
            imports: None,
            dynamic_imports: None,
        },
    );

    InternalManifest { bundles }
});

pub static COMPLEX_MANIFEST_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("src/parser/test_assets/assets/complex_manifest.json"));
pub const COMPLEX_MANIFEST_JSON_PRETTY: &str = include_str!("./assets/complex_manifest.json");
pub static COMPLEX_MANIFEST_CSS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    HashSet::from_iter(vec![
        "foo-5UjPuW-k.css".to_string(),
        "shared-ChJ_j-JJ.css".to_string(),
    ])
});
pub static COMPLEX_MANIFEST_JS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    HashSet::from_iter(vec![
        "baz-B2H3sXNv.js".to_string(),
        "bar-gkvgaI9m.js".to_string(),
        "shared-B7PI925R.js".to_string(),
        "foo-BRBmoGS9.js".to_string(),
    ])
});
pub static COMPLEX_MANIFEST: LazyLock<InternalManifest> = LazyLock::new(|| {
    let mut bundles = SIMPLE_MANIFEST.bundles.clone();

    bundles.insert(
        "_shared-B7PI925R.js".to_string(),
        InternalManifestDetails {
            css: Some(vec!["assets/shared-ChJ_j-JJ.css".to_string()]),
            file: "assets/shared-B7PI925R.js".to_string(),
            name: Some("shared".to_string()),
            src: None,
            is_dynamic_entry: None,
            is_entry: None,
            imports: None,
            dynamic_imports: None,
        },
    );

    // Skipping "_shared-v2JlcC3m.js" as it is found in the simple manifest.

    bundles.insert(
        "baz.js".to_string(),
        InternalManifestDetails {
            css: None,
            file: "assets/baz-B2H3sXNv.js".to_string(),
            name: Some("baz".to_string()),
            src: Some("baz.js".to_string()),
            is_dynamic_entry: Some(true),
            is_entry: None,
            imports: None,
            dynamic_imports: None,
        },
    );

    bundles.insert(
        "views/bar.js".to_string(),
        InternalManifestDetails {
            css: None,
            file: "assets/bar-gkvgaI9m.js".to_string(),
            name: Some("bar".to_string()),
            src: Some("views/bar.js".to_string()),
            is_dynamic_entry: None,
            is_entry: Some(true),
            imports: Some(vec!["_shared-B7PI925R.js".to_string()]),
            dynamic_imports: Some(vec!["baz.js".to_string()]),
        },
    );

    bundles.insert(
        "views/foo.js".to_string(),
        InternalManifestDetails {
            css: Some(vec!["assets/foo-5UjPuW-k.css".to_string()]),
            file: "assets/foo-BRBmoGS9.js".to_string(),
            name: Some("foo".to_string()),
            src: Some("views/foo.js".to_string()),
            is_dynamic_entry: None,
            is_entry: Some(true),
            imports: Some(vec!["_shared-B7PI925R.js".to_string()]),
            dynamic_imports: None,
        },
    );

    InternalManifest { bundles }
});
