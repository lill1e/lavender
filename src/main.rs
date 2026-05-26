use std::{env, fs};
mod structs;
use structs::lockfile::LockFile;

fn main() {
    let lockfile_data = fs::read_to_string(format!(
        "{}\\Riot Games\\Riot Client\\Config\\lockfile",
        env::var("LOCALAPPDATA").expect("Missing AppData environment variable")
    ))
    .expect("Riot Client lockfile missing");
    let lockfile =
        LockFile::new(lockfile_data).expect("There was an issue constructing the lockfile data");
}
