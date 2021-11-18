extern crate clap;

use clap::{App, Arg, ArgMatches};

use tempus::session::{Session, SessionStatus};
use tempus::utils;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn parse_args() -> ArgMatches<'static> {
    App::new("Tempus")
        .version("0.1")
        .author("Jake Wilson")
        .about("Easy time tracking")
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .value_name("PROJECT")
            .help("project name")
            .takes_value(true)
            .required(true))
        .get_matches()
}

fn main() {
    let matches = parse_args();
    let project = matches.value_of("project").unwrap();

    let project_dir_path = format!("{}/{}/{}", utils::get_home_dir(), TEMPUS_DIR_NAME, &project);
    utils::create_dir(&project_dir_path);

    let mut session = Session::new(&project_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(start_time) => {
            // TODO if both dates are the same, no need to print the date - just the times
            let end_time = session.end();
            session.record(&TEMPUS_LOG_NAME);

            let start_str = utils::format_datetime(&start_time);
            let end_str = utils::format_datetime(&end_time);
            // TODO add length in hours for session
            println!("{} session ended. {} to {}.", &project, &start_str, &end_str);
        },
        SessionStatus::NotStarted => {
            let start_time = session.start();
            println!("{} session started at {}.", &project, utils::format_datetime(&start_time));
        }
    };
}
