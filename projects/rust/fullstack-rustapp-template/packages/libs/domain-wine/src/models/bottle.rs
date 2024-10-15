use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync + 'static>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
}

/// An individual wine bottle
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WineBottle {
    id: String,
    name: String,
    vintage: String,
    winery: String,
    storage: String,
    created_at: String,
    updated_at: String,
}

impl WineBottle {
    pub fn new(
        id: String,
        name: String,
        vintage: String,
        winery: String,
        storage: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            name,
            vintage,
            winery,
            storage,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn vintage(&self) -> &str {
        &self.vintage
    }

    pub fn winery(&self) -> &str {
        &self.winery
    }

    pub fn storage(&self) -> &str {
        &self.storage
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}
