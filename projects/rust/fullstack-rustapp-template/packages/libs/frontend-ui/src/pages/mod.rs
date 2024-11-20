mod application;
mod router;
pub use router::get_router;

#[non_exhaustive]
pub struct ApplicationPages;

#[non_exhaustive]
pub struct StaticDirectories;

impl StaticDirectories {
    pub const VITE_ASSETS: &'static str = "/assets";
}