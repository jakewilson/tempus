extern crate chrono;

pub mod utils;
pub mod session;

use session::{Session, SessionStatus};

use std::fs::File;
use std::io::Read;
use std::path::Path;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn get_project_dir_path(project: &str) -> String {
    format!("{}/{}/{}", utils::get_home_dir(), &TEMPUS_DIR_NAME, &project)
}

pub fn calc_total_log_time(project: &str) {
    let log_file_path_str = format!("{}/{}", get_project_dir_path(project), &TEMPUS_LOG_NAME);
    let log_file_path = Path::new(&log_file_path_str);

    let mut file = match File::open(&log_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error opening {}: {}", log_file_path.display(), e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("error reading {}: {}", log_file_path.display(), e);
        std::process::exit(1);
    }

    let mut total_length_hours = 0.0;

    for i in contents.split('\n') {
        let hi: Vec<&str> = i.split(',').collect();

        if hi.len() < 2 {
            break;
        }

        let start = utils::datetime_from_str(hi[0]);
        let end = utils::datetime_from_str(hi[1]);

        total_length_hours += utils::get_length_hours(&start, &end);
    }

    println!("{:.3} hours", total_length_hours);
}

pub fn do_session(project: &str) {
    let project_dir_path = get_project_dir_path(project);
    utils::create_dir(&project_dir_path);

    let mut session = Session::new(&project_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(start_time) => {
            let end_time = session.end();
            session.record(&TEMPUS_LOG_NAME);

            let length_hours = format!("{:.3}", utils::get_length_hours(&start_time, &end_time));

            println!("{} session ended: {} hours.", &project, &length_hours);
        },
        SessionStatus::NotStarted => {
            let start_time = session.start();
            println!("{} session started at {}.", &project, utils::format_datetime(&start_time));
        }
    };
}
