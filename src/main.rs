use tempus::session::{Session, SessionStatus};
use tempus::utils;

const TEMPUS_DIR_NAME: &str = "/tempus/";
const SESSION_NAME: &str = ".session";
const TEMPUS_LOG_NAME: &str = "tempus_log.txt";

fn main() {
    // TODO create tempus config if it doesn't exist

    // TODO
    // grab the -p argument
    // check if the the `tempus_dir`/`project_name` dir exists
    // create it if it doesn't

    // create $HOME/tempus/ directory for storing sessions
    let tempus_dir_path = format!("{}/{}", utils::get_home_dir(), TEMPUS_DIR_NAME);
    utils::create_dir(&tempus_dir_path);

    let mut session = Session::new(&tempus_dir_path, SESSION_NAME);
    match session.status {
        SessionStatus::Started(start_time) => {
            let end_time = session.end();
            session.record(&TEMPUS_LOG_NAME);
            println!("Session ended. {} to {}.", utils::format_datetime(&start_time), utils::format_datetime(&end_time));
        },
        SessionStatus::NotStarted => {
            let start_time = session.start();
            println!("Session started at {}.", utils::format_datetime(&start_time));
        }
    };
}
