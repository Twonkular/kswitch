use crate::theme::Theme;
use chrono::NaiveTime;
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
        dbg!(&theme);
        theme.unwrap_or_else(|| sorted.last().unwrap().theme.clone())
    }
}
