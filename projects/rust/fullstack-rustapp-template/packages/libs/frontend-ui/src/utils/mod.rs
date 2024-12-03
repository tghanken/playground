use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use axum_extra::routing::TypedPath;

use crate::pages::StaticRoutes;

pub mod languages;

pub(crate) struct TypedRoute(String);

impl TypedRoute {
    pub fn new(path: impl TypedPath) -> TypedRoute {
        TypedRoute(path.to_string())
    }
}

impl Display for TypedRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
