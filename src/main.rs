use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    // TODO create tempus config if it doesn't exist

    // create $HOME/tempus/ directory for storing sessions
    let tempus_dir = create_tempus_dir();
    println!("dir {} is ready", tempus_dir);
}

fn get_home_dir() -> Result<String, env::VarError> {
    env::var("HOME")
}

/// Creates the tempus directory if it doesn't exist for
/// storing sessions
fn create_tempus_dir() -> String {
    // ~ is the home dir we use if $HOME does not exist
    let mut home_dir = String::from("~");

    if let Ok(dir) = get_home_dir() {
        home_dir = dir;
    }

    let tempus_dir = format!("{}/tempus/", home_dir);
    fs::create_dir(&tempus_dir).unwrap_or_else(|error| {
        // if it already exists, no problem
        if error.kind() != ErrorKind::AlreadyExists {
            panic!("could not create $HOME/tempus/ directory: {}", error);
        }
    });

    tempus_dir
}
