use std::env;

use std::io::ErrorKind;

use std::fs::{self, File, Metadata, OpenOptions};
use std::time::SystemTime;

/// Get the created time in seconds or panic
pub fn get_metadata_created_secs(metadata: Metadata) -> u64 {
    match metadata.created() {
        Ok(created_at) => get_system_time_secs(&created_at),
        Err(e) => panic!("err getting session metadata: {:?}", e),
    }
}

/// Convert a SystemTime to seconds or panic
pub fn get_system_time_secs(time: &SystemTime) -> u64 {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(e) => panic!("error getting SystemTime seconds: {}", e),
    }
}

/// Return the value of $HOME or panic if it doesn't exist
pub fn get_home_dir() -> String {
    match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(e) => panic!("error getting $HOME env variable: {}", e),
    }
}

/// Create a directory if it doesn't exist & returns the name
/// Panic if an error occurs while creating the dir
pub fn create_dir(dir: &str) {
    fs::create_dir(&dir).unwrap_or_else(|e| {
        // if it already exists, no problem
        if e.kind() != ErrorKind::AlreadyExists {
            panic!("could not create {} directory: {}", dir, e);
        }
    });
}

/// Open a file for appending or create it if it doesn't exist
/// Panic on error, return the file handle
pub fn create_or_open_file(path: &str) -> File {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path);

    match file {
        Ok(file) => file,
        Err(e) => panic!("Error opening {}: {}", &path, e),
    }
}

