use std::env;

use chrono::{NaiveDateTime, Duration, Timelike};

const HOUR_DURATION: Duration = Duration::hours(1);

pub struct HourIter {
    datetime: NaiveDateTime,
    remaining: i32,
}

impl HourIter {
    pub fn new(datetime: NaiveDateTime, count: u16) -> Self {
        HourIter { datetime, remaining: count as i32}
    }
}

// TODO: How to rust doc?
// Returns the next hour, starts a 0
// HourIter::new(date, 0) yields the date and completes
impl Iterator for HourIter {
    type Item = (chrono::NaiveDate, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining < 0 {
            return None;
        }
        let result = (self.datetime.date(), self.datetime.time().hour() as u16);
        self.remaining -= 1;
        self.datetime += HOUR_DURATION;
        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hours() {
        let expected = vec![
            ("2025-09-10".to_string(), 8u16),
            ("2025-09-10".to_string(), 9u16),
            ("2025-09-10".to_string(), 10u16),
            ("2025-09-10".to_string(), 11u16),
            ("2025-09-10".to_string(), 12u16),
            ("2025-09-10".to_string(), 13u16),
            ("2025-09-10".to_string(), 14u16),
            ("2025-09-10".to_string(), 15u16),
            ("2025-09-10".to_string(), 16u16),
            ("2025-09-10".to_string(), 17u16),
            ("2025-09-10".to_string(), 18u16),
            ("2025-09-10".to_string(), 19u16),
            ("2025-09-10".to_string(), 20u16),
            ("2025-09-10".to_string(), 21u16),
            ("2025-09-10".to_string(), 22u16),
            ("2025-09-10".to_string(), 23u16),
            ("2025-09-11".to_string(), 0u16),
            ("2025-09-11".to_string(), 1u16),
            ("2025-09-11".to_string(), 2u16),
            ("2025-09-11".to_string(), 3u16),
            ("2025-09-11".to_string(), 4u16),
        ];

        let start = NaiveDateTime::parse_from_str("2025-09-10 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let hours: Vec<_> = HourIter::new(start, expected.len() as u16 - 1).collect();
        // - 1 because HourIter::new(date, 0) generates exactly one item

        let actual: Vec<_> = hours
            .into_iter()
            .map(|(d, h)| (d.format("%Y-%m-%d").to_string(), h))
            .collect();

        assert_eq!(expected, actual);
    }
}

pub fn get_bool_env(flag: &str) -> bool {
    let val = env::var(flag).unwrap_or_else(|_| "false".to_string());
    let flag: bool = matches!(val.to_lowercase().as_str(), "1" | "true" | "yes" | "on");
    flag
}
