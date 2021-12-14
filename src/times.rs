use chrono::{DateTime, FixedOffset};

use std::vec::IntoIter;

use crate::utils;

#[derive(Debug)]
pub struct DateRange(pub DateTime<FixedOffset>, pub DateTime<FixedOffset>);

/// stores time content of the
/// TEMPUS_LOG file
#[derive(Debug)]
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
                DateRange(
                    utils::datetime_from_str(v[0]),
                    utils::datetime_from_str(v[1]),
                )
            })
            .collect::<Vec<_>>();

        if let Some(DateRange(start, end)) = date_range {
            let start_ms = start.timestamp();
            let end_ms = end.timestamp();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn different_timezones() {
        // top two are in est, bottom is utc
        // they should all be considered the same day
        let content = String::from(
            "2021-12-10T09:59:28-05:00,2021-12-10T10:36:14.153827-05:00
2021-12-10T11:16:09-05:00,2021-12-10T12:49:13.180945-05:00
2021-12-11T01:01:09+00:00,2021-12-11T02:00:13.180945+00:00",
        );

        let range: Option<DateRange> = Some(DateRange(
            utils::datetime_from_str("2021-12-10T00:00:00-05:00"),
            utils::datetime_from_str("2021-12-10T23:59:59-05:00"),
        ));

        let times = Times::new(&content, &range);

        assert_eq!(times.session_times.len(), 3);
    }

    #[test]
    fn different_timezones_2() {
        // top two are in est, bottom is utc
        // they should all be considered the same day
        let content = String::from(
            "2021-12-10T09:59:28-05:00,2021-12-10T10:36:14.153827-05:00
2021-12-10T11:16:09-05:00,2021-12-10T12:49:13.180945-05:00
2021-12-10T01:01:09+00:00,2021-12-10T02:00:13.180945+00:00",
        );

        let range: Option<DateRange> = Some(DateRange(
            utils::datetime_from_str("2021-12-10T00:00:00-05:00"),
            utils::datetime_from_str("2021-12-10T23:59:59-05:00"),
        ));

        let times = Times::new(&content, &range);

        assert_eq!(times.session_times.len(), 2);
    }
}
