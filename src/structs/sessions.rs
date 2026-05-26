use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionLaunchConfiguration {
    arguments: Vec<String>,
    executable: String,
    locale: Option<String>,
    working_directory: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionPatchlineFullName {
    #[serde(rename = "VALORANT")]
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionPatchlineId {
    #[serde(rename = "")]
    Empty,
    Live,
    Pbe,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionProductId {
    Valorant,
    RiotClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    exit_code: u64,
    is_internal: bool,
    launch_configuration: SessionLaunchConfiguration,
    patchline_full_name: SessionPatchlineFullName,
    patchline_id: SessionPatchlineId,
    phase: String,
    product_id: SessionProductId,
    version: String,
}

impl LockFile {
    pub fn sessions(&self) -> Result<HashMap<String, Session>> {
        self.get("/product-session/v1/external-sessions")
    }
}
