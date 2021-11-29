use chrono::{DateTime, FixedOffset};

use std::time::SystemTime;
use std::vec::IntoIter;

use crate::utils;

/// stores time content of the
/// TEMPUS_LOG file
pub struct Times<'a> {
    session_times: IntoIter<&'a str>,
}

impl<'a> Times<'_> {
    pub fn new(content: &'a str, today_only: bool) -> Times<'a> {
        let mut lines: Vec<&str> = content.split("\n").collect();
        if today_only {
            let now = utils::system_time_to_local_datetime(&SystemTime::now());
            let now_str = format!("{:?}", &now);
            // take an rfc_3339 date string like
            // 2021-11-28T18:28:38-05:00
            // and split it at 'T' to get the yyyy-mm-dd
            // call next() because we want the first match
            match now_str.split("T").next() {
                Some(date) => lines.retain(|x| x.contains(date)),
                None => eprintln!("Could not filter dates to today"),
            }
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
