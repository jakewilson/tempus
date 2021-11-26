extern crate clap;

use clap::{App, Arg, ArgMatches};

use taevus;

fn main() {
    let matches = parse_args();

    // this arg is required, so it's safe to unwrap
    let project = matches.value_of("project").unwrap();

    if matches.is_present("hours") {
        taevus::calc_total_log_time(&project);
    } else if matches.is_present("start") {
        taevus::print_session_start(&project);
    } else {
        taevus::do_session(&project);
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Taevus")
        .version("1.0")
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
            .help("calculate hours worked for a project"))
        .arg(Arg::with_name("start")
            .short("s")
            .long("start")
            .help("print current session start time"))
        .get_matches()
}

