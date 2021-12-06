use chrono::{DateTime, FixedOffset, Local, TimeZone};

use std::env;

use std::io::{ErrorKind, Read};

use std::fs::{self, File, Metadata, OpenOptions};
use std::path::Path;
use std::time::SystemTime;

use regex::Regex;

/// Get the created time or panic
pub fn get_metadata_created(metadata: Metadata) -> DateTime<Local> {
    match metadata.created() {
        Ok(created_at) => system_time_to_local_datetime(&created_at),
        Err(e) => panic!("err getting session metadata: {:?}", e),
    }
}

/// Convert a SystemTime to chrono::DateTime
pub fn system_time_to_local_datetime(time: &SystemTime) -> DateTime<Local> {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => Local.timestamp(duration.as_secs() as i64, 0),
        Err(e) => panic!("error getting SystemTime seconds: {}", e),
    }
}

pub fn format_datetime(time: &DateTime<Local>) -> String {
    time.to_rfc3339()
}

pub fn datetime_from_str(time: &str) -> DateTime<FixedOffset> {
    match DateTime::parse_from_rfc3339(time) {
        Ok(datetime) => datetime,
        Err(e) => panic!("failed to parse datetime {}: {}", time, e),
    }
}

/// Return the value of $HOME or panic if it doesn't exist
pub fn get_home_dir() -> String {
    match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(e) => panic!("error getting $HOME env variable: {}", e),
    }
}

/// Create a directory & all parent directories if they don't exist
/// & return the name. Panic if an error occurs while creating the dir
pub fn create_dir(dir: &str) {
    fs::create_dir_all(&dir).unwrap_or_else(|e| {
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

/// Returns the length in hours between the start & end time
pub fn get_length_hours<Tz: TimeZone>(start: &DateTime<Tz>, end: &DateTime<Tz>) -> f64 {
    ((end.timestamp() - start.timestamp()) as f64) / 3600.0
}

pub fn get_file_contents(path: &Path) -> String {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error opening {}: {}", path.display(), e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("error reading {}: {}", path.display(), e);
        std::process::exit(1);
    }

    contents
}

// TODO may be able to change this to a Tz: TimeZone generic param
// instead of fixedoffset
pub fn datetime_to_readable_str(date: &DateTime<FixedOffset>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_start_date() -> DateTime<Local> {
    Local.ymd(1970, 1, 1).and_hms(0, 0, 0)
}

pub fn get_todays_date() -> DateTime<Local> {
    system_time_to_local_datetime(&SystemTime::now())
}

pub fn get_date_from_arg(date_arg: &str) -> DateTime<Local> {
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();

    let caps = re
        .captures(date_arg)
        .expect(&format!("{} is not a valid date", date_arg));

    let year:  i32 = caps[1].parse().unwrap();
    let month: u32 = caps[2].parse().unwrap();
    let day:   u32 = caps[3].parse().unwrap();

    Local.ymd(year, month, day).and_hms(0, 0, 0)
}
