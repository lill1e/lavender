use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entitlements {
    /** Used as the token in requests */
    access_token: String,
    entitlements: Vec<String>,
    issuer: String,
    /** Player UUID */
    subject: String,
    /** Used as the entitlement in requests */
    token: String,
}

impl LockFile {
    pub fn entitlements(&self) -> Result<Entitlements> {
        self.get("/entitlements/v1/token")
    }
}
