use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
struct Session {
    federated: bool,
    game_name: String,
    game_tag: String,
    loaded: bool,
    name: String,
    pid: String,
    /** Player UUID */
    puuid: String,
    region: String,
    resource: String,
    state: String,
}

impl LockFile {
    fn session(&self) -> Result<Session> {
        self.get("/chat/v1/session")
    }
}
