pub use router::get_router;

pub mod application;
pub(crate) mod router;

#[non_exhaustive]
pub struct ApplicationPages;

#[non_exhaustive]
pub struct StaticRoutes;

impl StaticRoutes {
    pub const VITE_ASSETS: &'static str = "/assets";
}
