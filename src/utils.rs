use crate::times::DateRange;

use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone};

use std::env;

use std::io::{ErrorKind, Read};

use std::fs::{self, File, Metadata, OpenOptions};
use std::path::Path;
use std::time::SystemTime;

use regex::Regex;

/// Get the created time or panic
pub fn get_metadata_created(metadata: Metadata) -> DateTime<FixedOffset> {
    match metadata.created() {
        Ok(created_at) => system_time_to_datetime(&created_at),
        Err(e) => panic!("err getting session metadata: {:?}", e),
    }
}

/// Convert a SystemTime to chrono::DateTime
pub fn system_time_to_datetime(time: &SystemTime) -> DateTime<FixedOffset> {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => local_to_fixed_offset(Local.timestamp(duration.as_secs() as i64, 0)),
        Err(e) => panic!("error getting SystemTime seconds: {}", e),
    }
}

pub fn format_datetime(time: &DateTime<FixedOffset>) -> String {
    time.to_rfc3339()
}

pub fn datetime_from_str(time: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(time)
        .unwrap_or_else(|e| panic!("failed to parse datetime {}: {}", time, e))
}

/// Return the value of $HOME or panic if it doesn't exist
pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|e| panic!("error getting $HOME env variable: {}", e))
}

/// Create a directory & all parent directories if they don't exist
/// & return the name. Panic if an error occurs while creating the dir
pub fn create_dir(dir: &str) {
    fs::create_dir_all(&dir).unwrap_or_else(|e| {
        // if it already exists, no problem
        if e.kind() != ErrorKind::AlreadyExists {
            panic!("could not create {} directory: {}", dir, e);
        }
    });
}

/// Open a file for appending or create it if it doesn't exist
/// Panic on error, return the file handle
pub fn create_or_open_file(path: &str) -> File {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect(&format!("Error opening {}", &path))
}

/// Returns the length in hours between the start & end time
pub fn get_length_hours(start: &DateTime<FixedOffset>, end: &DateTime<FixedOffset>) -> f64 {
    ((end.timestamp() - start.timestamp()) as f64) / 3600.0
}

pub fn get_file_contents(path: &Path) -> String {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error opening {}: {}", path.display(), e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("error reading {}: {}", path.display(), e);
        std::process::exit(1);
    }

    contents
}

// TODO may be able to change this to a Tz: TimeZone generic param
// instead of fixedoffset
pub fn datetime_to_readable_str(date: &DateTime<FixedOffset>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_start_date() -> DateTime<FixedOffset> {
    local_to_fixed_offset(Local.ymd(1970, 1, 1).and_hms(0, 0, 0))
}

pub fn get_date_from_arg(date_arg: &str) -> DateTime<FixedOffset> {
    let re = Regex::new(r"^(\d{4}-)?(\d{1,2})-(\d{1,2})$").unwrap();

    let caps = re
        .captures(date_arg)
        .expect(&format!("{} is not a valid date", date_arg));

    let year = match caps.get(1) {
        // 0..4 is safe because if it exists, it _must_
        // look like `YYYY-` -- just remove the dash
        Some(_) => caps[1][0..4].parse().unwrap(),
        // if no year is provided, use this year
        None => Local::today().year(),
    };

    let month: u32 = caps[2].parse().unwrap();
    let day: u32 = caps[3].parse().unwrap();

    // if it's an 'end' date can do `and_hms(23, 59, 59)` for inclusivity
    local_to_fixed_offset(Local.ymd(year, month, day).and_hms(0, 0, 0))
}

pub fn local_to_fixed_offset(date: DateTime<Local>) -> DateTime<FixedOffset> {
    // why not just DateTime::from(date), you ask?
    // converting a date to a fixed offset using the from trait
    // also converts the timezone to utc, but we want to conserve
    // the timezone. There's probably an easier way to do this,
    // but this seems like the quickest to me after spending
    // a few hours reading the docs
    DateTime::parse_from_rfc3339(&date.to_rfc3339()).unwrap()
}

/// parses string in <date>(..(<date>)?)? format
/// where date -> 'today' | yyyy-mm-dd | mm-dd
/// <date> returns the range (<earliest_tempus_date>, <date>), inclusive
/// <date>.. returns the range (<date>, <today>), inclusive
/// <date1>..<date2> returns the range (<date1>, <date2>), inclusive
/// 'today' can be used in place of a date instead of typing today's date
/// a date without the year will search for this year
pub fn parse_date_range(date_range: &str) -> Result<DateRange, &str> {
    let dates = date_range.split("..").collect::<Vec<&str>>();

    let start_date = get_start_date();
    let todays_date = local_to_fixed_offset(Local::now());

    if dates.len() == 1 {
        // no dots (-d <date>), so this is the end date
        Ok(DateRange(start_date, get_date_from_arg(dates[0])))
    } else if dates.len() == 2 {
        match (dates[0], dates[1]) {
            ("", "") => Err("Invalid date-range provided"),
            ("", _) => Ok(DateRange(start_date, get_date_from_arg(dates[1]))),
            (_, "") => Ok(DateRange(get_date_from_arg(dates[0]), todays_date)),
            (_, _) => Ok(DateRange(
                get_date_from_arg(dates[0]),
                get_date_from_arg(dates[1]),
            )),
        }
    } else {
        Err("Invalid date-range provided")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_range() {
        let DateRange(start, end) = parse_date_range("2021-12-01..2021-12-13").unwrap();

        assert_eq!(
            Local.ymd(2021, 12, 1).and_hms(0, 0, 0).timestamp(),
            start.timestamp()
        );

        assert_eq!(
            Local.ymd(2021, 12, 13).and_hms(0, 0, 0).timestamp(),
            end.timestamp()
        );
    }

    #[test]
    fn test_date_range_end_only() {
        let DateRange(start, end) = parse_date_range("2021-12-01").unwrap();

        assert_eq!(
            Local.ymd(1970, 1, 1).and_hms(0, 0, 0).timestamp(),
            start.timestamp()
        );

        assert_eq!(
            Local.ymd(2021, 12, 1).and_hms(0, 0, 0).timestamp(),
            end.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg() {
        let d = get_date_from_arg("2021-12-01");

        assert_eq!(
            Local.ymd(2021, 12, 1).and_hms(0, 0, 0).timestamp(),
            d.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg_one_digit_day() {
        let d = get_date_from_arg("2021-12-1");

        assert_eq!(
            Local.ymd(2021, 12, 1).and_hms(0, 0, 0).timestamp(),
            d.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg_one_digit_month() {
        let d = get_date_from_arg("2021-1-10");

        assert_eq!(
            Local.ymd(2021, 1, 10).and_hms(0, 0, 0).timestamp(),
            d.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg_one_digit_month_and_day() {
        let d = get_date_from_arg("2021-1-1");

        assert_eq!(
            Local.ymd(2021, 1, 1).and_hms(0, 0, 0).timestamp(),
            d.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg_no_year() {
        let d = get_date_from_arg("12-01");

        assert_eq!(
            Local
                .ymd(Local::today().year(), 12, 1)
                .and_hms(0, 0, 0)
                .timestamp(),
            d.timestamp()
        );
    }

    #[test]
    fn test_get_date_from_arg_no_year_one_digit_day() {
        let d = get_date_from_arg("12-5");

        assert_eq!(
            Local
                .ymd(Local::today().year(), 12, 5)
                .and_hms(0, 0, 0)
                .timestamp(),
            d.timestamp()
        );
    }
}
