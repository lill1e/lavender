use std::fmt::Display;

use anyhow::{Result, anyhow};
use ureq::{config::Config, tls::TlsConfig};

use crate::structs::callable::Callable;

#[derive(Debug, Clone)]
pub struct LockFile {
    name: String,
    pid: u32,
    pub port: u16,
    pub password: String,
    protocol: String,
}

impl LockFile {
    pub fn new(lockfile: String) -> Result<LockFile> {
        let mut split_strs = lockfile.split(":").map(|s| s.to_owned());
        Ok(LockFile {
            name: split_strs
                .next()
                .ok_or(anyhow!("Insufficient amount of values"))?,
            pid: split_strs
                .next()
                .ok_or(anyhow!("Insufficient amount of values"))?
                .parse()?,
            port: split_strs
                .next()
                .ok_or(anyhow!("Insufficient amount of values"))?
                .parse()?,
            password: split_strs
                .next()
                .ok_or(anyhow!("Insufficient amount of values"))?,
            protocol: split_strs
                .next()
                .ok_or(anyhow!("Insufficient amount of values"))?,
        })
    }
}

impl Display for LockFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LockFile {{
    name: {}
    pid: {}
    port: {}
    password: {}
    protocol: {}\n}}",
            self.name, self.pid, self.port, self.password, self.protocol
        )
    }
}

impl Callable for LockFile {
    fn agent(&self) -> ureq::Agent {
        ureq::Agent::new_with_config(
            Config::builder()
                .tls_config(TlsConfig::builder().disable_verification(true).build())
                .build(),
        )
    }
}
