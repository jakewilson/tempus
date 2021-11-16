use std::io::{ErrorKind, Write};
use std::fs::{self, File};
use std::time::SystemTime;

extern crate chrono;
use chrono::{Local, TimeZone};

use crate::utils;

#[derive(Debug)]
pub enum SessionStatus {
    Started(u64),
    NotStarted,
}

#[derive(Debug)]
pub struct Session<'a> {
    started_at_secs: Option<u64>,
    ended_at_secs: Option<u64>,
    session_dir: &'a str,
    session_path: String,
    pub status: SessionStatus,
    session_name: &'a str,
}

impl Session<'_> {
    pub fn new<'a>(session_dir: &'a str, session_name: &'a str) -> Session<'a> {
        let session_path = format!("{}/{}", session_dir, session_name);
        let status = Session::get_status(&session_path);
        let started_at_secs = match status {
            SessionStatus::Started(secs) => Some(secs),
            SessionStatus::NotStarted => None,
        };

        Session {
            started_at_secs,
            ended_at_secs: None,
            session_dir,
            status, 
            session_path,
            session_name,
        }
    }

    /// Try to retrieve metadata on the session file
    /// If it exists, that means the session has already been started
    /// If the file doesn't exist, that means the session hasn't yet
    /// been started
    fn get_status(path: &str) -> SessionStatus {
        match fs::metadata(&path) {
            Ok(metadata) => SessionStatus::Started(utils::get_metadata_created_secs(metadata)),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    SessionStatus::NotStarted
                } else {
                    panic!("error getting session metadata: {}", e);
                }
            }
        }
    }

    /// Start the session
    pub fn start(&self) {
        if let SessionStatus::Started(_) = self.status {
            panic!("Tried to start a session that is already started.");
        }

        if let Err(e) = File::create(&self.session_path) {
            panic!("error creating session file: {}", e);
        }
    }

    /// End the session
    pub fn end(&mut self) {
        if let SessionStatus::NotStarted = self.status {
            panic!("Tried to end a session that hasn't been started.");
        }

        // delete tempus session file - don't need it anymore
        if let Err(e) = fs::remove_file(&self.session_path) {
            panic!("error removing session file: {}", e);
        }

        self.ended_at_secs = Some(utils::get_system_time_secs(&SystemTime::now()));
    }

    /// Creates the log file if it doesn't already exist
    /// Records the start, & end of the session
    pub fn record(&self, log_name: &str) {
        let log_file_path = format!("{}/{}", self.session_dir, log_name);
        let mut file = utils::create_or_open_file(&log_file_path);

        let start = self.started_at_secs.unwrap();
        let end = self.ended_at_secs.unwrap();

        let start_dt = Local.timestamp(start as i64, 0).format("%Y-%m-%d %H:%M:%S").to_string();
        let end_dt = Local.timestamp(end as i64, 0).format("%Y-%m-%d %H:%M:%S").to_string();
        let session_record = format!("{},{}\n", &start_dt, &end_dt);

        file.write(&session_record.as_bytes()).unwrap_or_else(|e| {
            let session_info = format!("start,end,length\n{}", &session_record);
            panic!("Error logging session:\n{}\nerror: {}", &session_info, e);
        });
    }
}
