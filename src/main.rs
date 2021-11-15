use std::env;

use std::fs::{self, File, Metadata};
use std::io::{self, ErrorKind, Write};

use std::time::SystemTime;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const TEMPUS_SESSION_NAME: &str = ".session";

fn main() {
    // TODO create tempus config if it doesn't exist

    // create $HOME/tempus/ directory for storing sessions
    let tempus_dir = create_tempus_dir();

    let tempus_session_path = format!("{}/{}", tempus_dir, TEMPUS_SESSION_NAME);

    // grab the -p argument
    // check if the the `tempus_dir`/`project_name` dir exists
    // create it if it doesn't

    // check if session file exists & grab metadata if it does
    let created_at_ms;
    // TODO maybe add an enum like `SESSION_STARTED` and `SESSION_ENDED(u128)`
    // & return the enum from a fn to clean this up a little?
    // fn name get_created_at_ms_or_start_session? idk
    match fs::metadata(&tempus_session_path) {
        Ok(metadata) => created_at_ms = get_created_at_ms(metadata),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                start_session(&tempus_session_path);
            } else {
                panic!("error getting session metadata: {}", e);
            }
            return;
        }
    }

    println!("created_at: {}", created_at_ms);

    // delete tempus session file - don't need it anymore
    if let Err(e) = fs::remove_file(&tempus_session_path) {
        panic!("error removing session file: {}", e);
    }

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

/// Get the created time in ms or panic
fn get_created_at_ms(metadata: Metadata) -> u128 {
    match metadata.created() {
        Ok(created) => match created.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(e) => panic!("error getting created_at millis: {}", e),
        },
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

