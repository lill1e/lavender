use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSessionLaunchConfiguration {
    arguments: Vec<String>,
    executable: String,
    locale: Option<String>,
    working_directory: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSessionPatchlineFullName {
    #[serde(rename = "VALORANT")]
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSessionPatchlineId {
    #[serde(rename = "")]
    Empty,
    Live,
    Pbe,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSessionProductId {
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSession {
    exit_code: u64,
    is_internal: bool,
    launch_configuration: ExternalSessionLaunchConfiguration,
    patchline_full_name: ExternalSessionPatchlineFullName,
    patchline_id: ExternalSessionPatchlineId,
    phase: String,
    product_id: ExternalSessionProductId,
    version: String,
}

impl LockFile {
    pub fn sessions(&self) -> Result<HashMap<String, ExternalSession>> {
        self.get("/product-session/v1/external-sessions")
    }
}
