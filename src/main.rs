extern crate clap;

use clap::{App, Arg, ArgMatches};

use tempus_cli;

fn main() {
    let matches = parse_args();

    // this arg is required, so it's safe to unwrap
    let project = matches.value_of("project").unwrap();

    if matches.is_present("hours") {
        tempus_cli::print_total_log_time(&project, matches.is_present("today"));
    } else if matches.is_present("start") {
        tempus_cli::print_session_start(&project);
    } else if matches.is_present("times") {
        tempus_cli::print_times(&project, matches.is_present("today"));
    } else {
        tempus_cli::do_session(&project);
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Tempus")
        .version("1.1.0")
        .author("Jake Wilson")
        .about("Easy time tracking")
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .value_name("PROJECT")
            .help("project name")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("hours")
            .long("hours")
            .help("Calculates hours worked for a project"))
        .arg(Arg::with_name("start")
            .short("s")
            .long("start")
            .help("Prints current session start time"))
        .arg(Arg::with_name("today")
            .long("today")
            .help("Filters session times to those from today. Use with --hours or --times"))
        .arg(Arg::with_name("times")
            .long("times")
            .help("Prints all session times for a project"))
        .get_matches()
}

