use crate::theme::Theme;
use chrono::{Datelike, Local, NaiveDate, Timelike, Utc};
use std::env;
use std::str::FromStr;

use crate::config::Config;

fn get_from_time(config: &Config) -> Theme {
    let time = Local::now().naive_local().time();
    config.schedule.theme_from_time(&time)
}

pub fn get(config: &Config) -> Theme {
    let current = match env::var("KSWITCH_THEME") {
        Ok(theme) => {
            // Try to get the theme from environment_variable
            Theme::from_str(theme.as_str()).unwrap()
        }
        Err(e) => {
            // otherwise get from time of day/daylight hours
            get_from_time(config)
        }
    };

    match current {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let config = Config::default();
        let out = get(&config);
    }
}
