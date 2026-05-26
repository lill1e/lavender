use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    locale: String,
    region: String,
    web_language: String,
    web_region: String,
}

impl LockFile {
    pub fn region(&self) -> Result<Region> {
        self.get("/riotclient/region-locale")
    }
}
