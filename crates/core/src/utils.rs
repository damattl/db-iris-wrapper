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

pub fn get_bool_env(flag: &str) -> bool {
    let val = env::var(flag).unwrap_or_else(|_| "false".to_string());
    let flag: bool = matches!(val.to_lowercase().as_str(), "1" | "true" | "yes" | "on");
    flag
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
    }

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

    #[test]
    fn get_bool_env_treats_truthy_values_as_true() {
        let _guard = env_guard();
        for value in ["1", "true", "Yes", "ON"] {
            env::set_var("WRAPPER_CORE_BOOL", value);
            assert!(get_bool_env("WRAPPER_CORE_BOOL"), "value {value} should be truthy");
        }
        env::remove_var("WRAPPER_CORE_BOOL");
    }

    #[test]
    fn get_bool_env_defaults_to_false() {
        let _guard = env_guard();
        env::remove_var("WRAPPER_CORE_BOOL");
        assert!(!get_bool_env("WRAPPER_CORE_BOOL"));
        for value in ["0", "false", "off", "random"] {
            env::set_var("WRAPPER_CORE_BOOL", value);
            assert!(!get_bool_env("WRAPPER_CORE_BOOL"), "value {value} should be falsey");
        }
        env::remove_var("WRAPPER_CORE_BOOL");
    }
}
