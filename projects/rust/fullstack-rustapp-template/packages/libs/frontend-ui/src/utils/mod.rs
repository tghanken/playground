use std::collections::HashSet;

use crate::pages::StaticRoutes;

pub mod languages;

#[tracing::instrument]
pub fn make_asset_vector(paths: HashSet<String>) -> Vec<String> {
    let base_path = StaticRoutes::VITE_ASSETS;

    let mut assets = Vec::new();
    for path in paths {
        let full_path = format!("{base_path}/{}", path);
        assets.push(full_path);
    }
    assets
}
