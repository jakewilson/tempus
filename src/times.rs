use chrono::{DateTime, FixedOffset};

use std::vec::IntoIter;

use crate::utils;

#[derive(Debug)]
pub struct DateRange (pub DateTime<FixedOffset>, pub DateTime<FixedOffset>);

/// stores time content of the
/// TEMPUS_LOG file
pub struct Times {
    session_times: IntoIter<DateRange>,
}

impl Times {
    pub fn new(content: &str, date_range: &Option<DateRange>) -> Times {
        let lines: Vec<&str> = content.trim().split('\n').collect();
        let mut dates = lines
            .iter()
            .map(|line| {
                let v = line.split(',').take(2).collect::<Vec<&str>>();
                DateRange(utils::datetime_from_str(v[0]), utils::datetime_from_str(v[1]))
            })
            .collect::<Vec<DateRange>>();

        if let Some(DateRange(start, end)) = date_range {
            let start_ms = start.timestamp();
            let end_ms   = end.timestamp();
            dates.retain(|DateRange(session_start, _)| {
                let ms = session_start.timestamp();
                ms >= start_ms && ms <= end_ms
            });
        }

        Times {
            session_times: dates.into_iter(),
        }
    }
}

impl Iterator for Times {
    type Item = DateRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.session_times.next()
    }
}
