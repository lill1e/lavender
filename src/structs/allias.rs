use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
pub struct Alias {
    active: bool,
    created_datetime: u64,
    game_name: String,
    summoner: bool,
    tag_line: String,
}

impl LockFile {
    pub fn alias(&self) -> Result<Alias> {
        self.get("/player-account/aliases/v1/active")
    }
}
