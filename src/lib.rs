extern crate chrono;

pub mod utils;
pub mod session;
pub mod times;

use session::{Session, SessionStatus};
use times::Times;

use std::path::Path;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn get_project_dir_path(project: &str) -> String {
    format!("{}/{}/{}", utils::get_home_dir(), &TEMPUS_DIR_NAME, &project)
}

pub fn print_total_log_time(project: &str, today_only: bool) {
    let log_file_path_str = format!("{}/{}", get_project_dir_path(project), &TEMPUS_LOG_NAME);
    let log_file_path = Path::new(&log_file_path_str);

    let contents = utils::get_file_contents(&log_file_path);
    let mut total_length_hours = 0.0;

    for line in contents.split('\n') {
        let times: Vec<&str> = line.split(',').collect();

        if times.len() < 2 {
            break;
        }

        let start = utils::datetime_from_str(times[0]);
        let end = utils::datetime_from_str(times[1]);

        total_length_hours += utils::get_length_hours(&start, &end);
    }

    println!("{:.3}", total_length_hours);
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

pub fn print_session_start(project: &str) {
    let project_dir_path = get_project_dir_path(project);

    let session = Session::new(&project_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(start_time) => println!("{}", utils::format_datetime(&start_time)),
        SessionStatus::NotStarted => eprintln!("No session started for {}", project),
    };
}

pub fn print_times(project: &str, today_only: bool) {
    let log_file_path_str = format!("{}/{}", get_project_dir_path(project), &TEMPUS_LOG_NAME);
    let log_file_path = Path::new(&log_file_path_str);

    let contents = utils::get_file_contents(&log_file_path);
    let times = Times::new(&contents, today_only);

    for (start, end) in times {
        println!("{}, {}",
            utils::datetime_to_readable_str(&start),
            utils::datetime_to_readable_str(&end)
        );
    }
}
