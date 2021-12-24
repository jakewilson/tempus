extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

use tempus_cli::times::DateRange;
use tempus_cli::utils;

fn main() {
    let matches = parse_args();

    // this arg is required, so it's safe to unwrap
    let project = matches.value_of("project").unwrap();

    if let Some(matches) = matches.subcommand_matches("log") {
        log(matches, &project);
    } else if let Some(matches) = matches.subcommand_matches("hours") {
        hours(matches, &project);
    } else {
        exec_main(&matches, &project);
    }
}

/// `log` subcommand
fn log(matches: &ArgMatches, project: &str) {
    let date_range = parse_date_range_arg(matches);
    tempus_cli::print_log(&project, &date_range);
}

/// `hours` subcommand
fn hours(matches: &ArgMatches, project: &str) {
    let date_range = parse_date_range_arg(matches);
    tempus_cli::print_total_log_time(&project, &date_range);
}

/// no subcommand
fn exec_main(matches: &ArgMatches, project: &str) {
    if matches.is_present("start") {
        tempus_cli::print_session_start(&project);
    } else if matches.is_present("delete") {
        tempus_cli::delete_session(&project);
    } else {
        tempus_cli::do_session(&project);
    }
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Tempus")
        .version("1.2.0")
        .author("Jake Wilson")
        .about("Easy time tracking")
        .arg(
            Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("project")
                .takes_value(true)
                .required(true)
                .help("project name"),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .help("Prints current session start time"),
        )
        .arg(
            Arg::with_name("delete")
                .short("x")
                .help("Ends the session without recording it"),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Lists tempus sessions")
                .arg(date_range_arg()),
        )
        .subcommand(
            SubCommand::with_name("hours")
                .about("Get session hours")
                .arg(date_range_arg()),
        )
        .get_matches()
}

fn date_range_arg() -> Arg<'static, 'static> {
    Arg::with_name("date-range")
        .value_name("date-range")
        .takes_value(true)
        .help(
            "Inclusive date range filter.
<start_date>..<end_date>
Date format: yyyy-mm-dd | mm-dd | 'today'",
        )
}

fn parse_date_range_arg(matches: &ArgMatches) -> Option<DateRange> {
    match matches.value_of("date-range") {
        Some(range) => match utils::parse_date_range(&range) {
            Ok(date_range) => Some(date_range),
            Err(err) => tempus_cli::exit(err),
        },
        None => None,
    }
}
