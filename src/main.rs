use std::env;

use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{ErrorKind, Write};

use std::time::SystemTime;

enum SessionStatus {
    Started(u64),
    NotStarted,
}

const TEMPUS_DIR_NAME: &str = "/tempus/";
const TEMPUS_SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn main() {
    // TODO create tempus config if it doesn't exist
    // TODO move session into a struct

    // create $HOME/tempus/ directory for storing sessions
    let tempus_dir = create_tempus_dir();

    let tempus_session_path = format!("{}/{}", tempus_dir, TEMPUS_SESSION_NAME);

    // grab the -p argument
    // check if the the `tempus_dir`/`project_name` dir exists
    // create it if it doesn't

    // check if session has been started or not & grab metadata if it does
    let session_started_at_secs = match get_session_status(&tempus_session_path) {
        SessionStatus::Started(millis) => millis,
        SessionStatus::NotStarted => {
            start_session(&tempus_session_path);
            println!("Session started.");
            // there is nothing left to do after
            // after starting the session
            return;
        }
    };

    let session_ended_at_secs = end_session(&tempus_session_path);
    record_session(session_started_at_secs, session_ended_at_secs, &tempus_dir);

    // calculate session end time

    // if it does:
    //  - delete the file
    //  - store `start,end` in a log file
}

/// Create session file or panic
fn start_session(path: &str) {
    if let Err(e) = File::create(&path) {
        panic!("error creating session file: {}", e);
    }
}

/// Get the created time in seconds or panic
fn get_session_start_secs(metadata: Metadata) -> u64 {
    match metadata.created() {
        Ok(created_at) => get_system_time_secs(&created_at),
        Err(e) => panic!("err getting session metadata: {:?}", e),
    }
}

/// Create the tempus directory if it doesn't exist for
/// storing sessions
fn create_tempus_dir() -> String {
    let tempus_dir = format!("{}/{}", get_home_dir(), TEMPUS_DIR_NAME);
    fs::create_dir(&tempus_dir).unwrap_or_else(|e| {
        // if it already exists, no problem
        if e.kind() != ErrorKind::AlreadyExists {
            panic!("could not create {} directory: {}", tempus_dir, e);
        }
    });

    tempus_dir
}

/// Return the value of $HOME or panic if it doesn't exist
fn get_home_dir() -> String {
    match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(e) => panic!("error getting $HOME env variable: {}", e),
    }
}

/// Try to retrieve metadata on the session file
/// If it exists, that means the session has already been started
/// If the file doesn't exist, that means the session hasn't yet
/// been started
fn get_session_status(path: &str) -> SessionStatus {
    match fs::metadata(&path) {
        Ok(metadata) => SessionStatus::Started(get_session_start_secs(metadata)),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                SessionStatus::NotStarted
            } else {
                panic!("error getting session metadata: {}", e);
            }
        }
    }
}

/// Convert a SystemTime to seconds or panic
fn get_system_time_secs(time: &SystemTime) -> u64 {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(e) => panic!("error getting SystemTime seconds: {}", e),
    }
}

/// End the session and return the end time in seconds
fn end_session(path: &str) -> u64 {
    // delete tempus session file - don't need it anymore
    if let Err(e) = fs::remove_file(&path) {
        panic!("error removing session file: {}", e);
    }

    get_system_time_secs(&SystemTime::now())
}

/// Creates the log file if it doesn't already exist
/// Records the start, end, & length of the newly
/// ended session
fn record_session(start: u64, end: u64, dir: &str) {
    let log_file_path = format!("{}/{}", dir, TEMPUS_LOG_NAME);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path);

    match file {
        Ok(mut file) => {
            let session_record = format!("{},{},{}\n", start, end, end - start);
            file.write(&session_record.as_bytes());
        },
        Err(e) => panic!("Error opening {}: {}", log_file_path, e),
    };
}
