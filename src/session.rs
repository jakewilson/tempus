use std::io::{ErrorKind, Write};
use std::fs::{self, File};

use chrono::{DateTime, FixedOffset, Local};

use crate::utils;

#[derive(Debug)]
pub enum SessionStatus {
    Started(DateTime<FixedOffset>),
    NotStarted,
}

#[derive(Debug)]
pub struct Session<'a> {
    started_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    session_dir: &'a str,
    session_path: String,
    pub status: SessionStatus,
    session_name: &'a str,
}

impl Session<'_> {
    pub fn new<'a>(session_dir: &'a str, session_name: &'a str) -> Session<'a> {
        let session_path = format!("{}/{}", session_dir, session_name);
        let status = Session::get_status(&session_path);
        let started_at = match status {
            SessionStatus::Started(secs) => Some(secs),
            SessionStatus::NotStarted => None,
        };

        Session {
            started_at,
            ended_at: None,
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
            Ok(metadata) => SessionStatus::Started(utils::get_metadata_created(metadata)),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    SessionStatus::NotStarted
                } else {
                    panic!("error getting session metadata: {}", e);
                }
            }
        }
    }

    /// Start the session & return the start time
    pub fn start(&self) -> DateTime<FixedOffset> {
        if let SessionStatus::Started(_) = self.status {
            panic!("Tried to start a session that is already started.");
        }

        match File::create(&self.session_path) {
            Ok(file) => match file.metadata() {
                Ok(metadata) => utils::get_metadata_created(metadata),
                Err(e) => panic!("error getting session start time: {}", e),
            },
            Err(e) => panic!("error creating session file: {}", e),
        }
    }

    /// End the session & return the end time
    pub fn end(&mut self) -> DateTime<FixedOffset> {
        if let SessionStatus::NotStarted = self.status {
            panic!("Tried to end a session that hasn't been started.");
        }

        // delete session file - don't need it anymore
        if let Err(e) = fs::remove_file(&self.session_path) {
            panic!("error removing session file: {}", e);
        }

        let ended_at: DateTime<FixedOffset> = DateTime::from(Local::now());

        // we want two copies - one to save to the session & one to return
        self.ended_at = Some(ended_at.clone());
        ended_at
    }

    /// Creates the log file if it doesn't already exist &
    /// records the start & end of the session
    pub fn record(&self, log_name: &str) {
        let log_file_path = format!("{}/{}", self.session_dir, log_name);
        let mut file = utils::create_or_open_file(&log_file_path);

        let start = self.started_at.unwrap();
        let end = self.ended_at.unwrap();

        let start_dt = utils::format_datetime(&start);
        let end_dt = utils::format_datetime(&end);
        let session_record = format!("{},{}\n", &start_dt, &end_dt);

        file.write(&session_record.as_bytes()).unwrap_or_else(|e| {
            let session_info = format!("start,end,length\n{}", &session_record);
            panic!("Error logging session:\n{}\nerror: {}", &session_info, e);
        });
    }
}
