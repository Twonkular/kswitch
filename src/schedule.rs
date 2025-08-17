use std::arch::x86_64::_mm_sha1nexte_epu32;

use crate::theme::Theme;
use chrono::{Local, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Window {
    theme: Theme,
    start: NaiveTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub windows: Vec<Window>,
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            windows: vec![
                Window {
                    theme: Theme::Light,
                    start: NaiveTime::from_hms_opt(7, 0, 0)
                        .expect("Failure to generate default start time"),
                },
                Window {
                    theme: Theme::Dark,
                    start: NaiveTime::from_hms_opt(17, 0, 0)
                        .expect("Failure to generate default start time"),
                },
            ],
        }
    }
}

impl Schedule {
    pub fn theme_from_time(&self, time: &NaiveTime) -> Theme {
        // Sort windows by start time (if not guaranteed)
        let mut sorted = self.windows.clone();
        sorted.sort_by_key(|w| w.start);

        // Find the last window that starts before or at current time
        let theme = sorted
            .iter()
            .rev()
            .find(|w| w.start <= *time)
            .map(|w| w.theme.clone());

        // If none matched, use the last window (wrap around midnight)
        theme.unwrap_or_else(|| sorted.last().unwrap().theme.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), Theme::Dark)]
    #[case(NaiveTime::from_hms_opt(10, 0, 0).unwrap(), Theme::Light)]
    #[case(NaiveTime::from_hms_opt(21, 0, 0).unwrap(), Theme::Dark)]
    fn test_theme_from_time(#[case] time: NaiveTime, #[case] theme: Theme) {
        let schedule = Schedule::default();
        let result = schedule.theme_from_time(&time);
        assert_eq!(result, theme);
    }
}
