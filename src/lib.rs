mod structs;

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::structs::lockfile::LockFile;

    #[test]
    fn it_works() {
        let lockfile_data = fs::read_to_string(format!(
            "{}\\Riot Games\\Riot Client\\Config\\lockfile",
            env::var("LOCALAPPDATA").expect("Missing AppData environment variable")
        ))
        .expect("Riot Client lockfile missing");
        let lockfile = LockFile::new(lockfile_data)
            .expect("There was an issue constructing the lockfile data");
        println!("{}", lockfile);
    }
}
