use std::{env, fmt::Display, fs};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
struct LockFile {
    name: String,
    pid: u32,
    port: u16,
    password: String,
    protocol: String,
}

impl LockFile {
    fn new(lockfile: String) -> Result<LockFile> {
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
    protocol: {}\n}}", self.name, self.pid, self.port, self.password, self.protocol
        )
    }
}

fn main() {
    let lockfile_data = fs::read_to_string(format!(
        "{}\\Riot Games\\Riot Client\\Config\\lockfile",
        env::var("LOCALAPPDATA").expect("Missing AppData environment variable")
    ))
    .expect("Riot Client lockfile missing");
    println!(
        "{}",
        LockFile::new(lockfile_data).expect("There was an issue constructing the lockfile data")
    );
}
