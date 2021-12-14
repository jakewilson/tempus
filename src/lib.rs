extern crate chrono;

pub mod session;
pub mod times;
pub mod utils;

use session::{Session, SessionStatus};
use times::{DateRange, Times};

use std::path::Path;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn get_project_dir_path(project: &str) -> String {
    format!(
        "{}/{}/{}",
        utils::get_home_dir(),
        &TEMPUS_DIR_NAME,
        &project
    )
}

pub fn print_total_log_time(project: &str, date_range: &Option<DateRange>) {
    let log_file_path_str = format!("{}/{}", get_project_dir_path(project), &TEMPUS_LOG_NAME);
    let log_file_path = Path::new(&log_file_path_str);

    let contents = utils::get_file_contents(&log_file_path);
    let times = Times::new(&contents, date_range);

    let total_length_hours = times.fold(0.0, |sum, DateRange(start, end)| {
        sum + utils::get_length_hours(&start, &end)
    });

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
        }
        SessionStatus::NotStarted => {
            let start_time = session.start();
            println!(
                "{} session started at {}.",
                &project,
                utils::datetime_to_readable_str(&start_time)
            );
        }
    };
}

pub fn delete_session(project: &str) {
    let project_dir_path = get_project_dir_path(project);

    let mut session = Session::new(&project_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(_) => {
            let end_time = session.end();

            println!(
                "{} session ended at {} and deleted.",
                &project,
                utils::datetime_to_readable_str(&end_time)
            );
        }
        SessionStatus::NotStarted => {
            eprintln!("No session started for {}.", &project);
        }
    };
}

pub fn print_session_start(project: &str) {
    let project_dir_path = get_project_dir_path(project);

    let session = Session::new(&project_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(start_time) => {
            println!("{}", utils::datetime_to_readable_str(&start_time))
        }
        SessionStatus::NotStarted => eprintln!("No session started for {}.", project),
    };
}

pub fn print_times(project: &str, date_range: &Option<DateRange>) {
    let log_file_path_str = format!("{}/{}", get_project_dir_path(project), &TEMPUS_LOG_NAME);
    let log_file_path = Path::new(&log_file_path_str);

    let contents = utils::get_file_contents(&log_file_path);
    let times = Times::new(&contents, date_range);

    for DateRange(start, end) in times {
        println!(
            "{}, {}",
            utils::datetime_to_readable_str(&start),
            utils::datetime_to_readable_str(&end)
        );
    }
}

pub fn exit(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}
