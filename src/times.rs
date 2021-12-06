use chrono::{DateTime, FixedOffset, Local};

use std::time::SystemTime;
use std::vec::IntoIter;

use crate::utils;

pub struct DateRange (pub DateTime<Local>, pub DateTime<Local>);

/// stores time content of the
/// TEMPUS_LOG file
pub struct Times<'a> {
    session_times: IntoIter<&'a str>,
}

impl<'a> Times<'_> {
    pub fn new(content: &'a str, date_range: &Option<DateRange>) -> Times<'a> {
        let mut lines: Vec<&str> = content.split('\n').collect();
        let mut dates = lines
            .iter()
            .map(|line| {
                let v = line.split(',').take(2).collect::<Vec<&str>>();
                (utils::datetime_from_str(v[0]), utils::datetime_from_str(v[1]))
            })
            .collect::<Vec<DateRange>>();

        dbg!(dates);
        if let Some(DateRange(start, end)) = date_range {
            let start_ms = start.timestamp();
            let end_ms   = end.timestamp();
            dates.retain(|(session_start, _)| {
                let ms = session_start.timestamp();
                ms >= start_ms && ms <= end_ms
            });
        }

        Times {
            session_times: lines.into_iter(),
        }
    }
}

impl Iterator for Times<'_> {
    type Item = (DateTime<FixedOffset>, DateTime<FixedOffset>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.session_times.next() {
            Some(range) => {
                let times_vec: Vec<&str> = range.split(",").collect();
                if times_vec.len() == 2 {
                    let start = utils::datetime_from_str(times_vec[0]);
                    let end = utils::datetime_from_str(times_vec[1]);
                    Some((start, end))
                } else {
                    None
                }
            },
            None => None,
        }
    }
}
