pub mod dashboard;
pub mod themes;

#[non_exhaustive]
pub struct ApplicationPages;

#[non_exhaustive]
pub struct StaticDirectories;

impl StaticDirectories {
    pub const VITE_ASSETS: &'static str = "/assets";
}