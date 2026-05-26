use std::fmt::Display;

use anyhow::Result;
use serde::Deserialize;

use crate::structs::{callable::Callable, lockfile::LockFile};

#[derive(Debug)]
pub enum Region {
    Na,
    Latam,
    Br,
    Eu,
    Ap,
    Kr,
}

#[derive(Debug)]
pub enum Shard {
    Na,
    Pbe,
    Eu,
    Ap,
    Kr,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Region::Na => "na",
                Region::Latam => "latam",
                Region::Br => "br",
                Region::Eu => "eu",
                Region::Ap => "ap",
                Region::Kr => "kr",
            }
        )
    }
}

impl Display for Shard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shard::Na => "na",
                Shard::Pbe => "pbe",
                Shard::Eu => "eu",
                Shard::Ap => "ap",
                Shard::Kr => "kr",
            }
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionData {
    locale: String,
    region: String,
    web_language: String,
    web_region: String,
}

impl LockFile {
    pub fn region(&self) -> Result<RegionData> {
        self.get("/riotclient/region-locale")
    }
}
