use crate::theme::Theme;
use chrono::{Datelike, Local, NaiveDate, Timelike, Utc};
use std::env;
use std::str::FromStr;

use crate::config::Config;

fn get_theme_from_schedule(config: &Config) -> Theme {
    let time = Local::now().naive_local().time();
    config.schedule.theme_from_time(&time)
}

/// Gets the current kswitch theme, either by reading the environment variable, if it exits, otherwise it is determined from the time of dat and the schedule defined in config.
pub fn get(config: &Config) -> Theme {
    match env::var("KSWITCH_THEME") {
        Ok(theme) => {
            // Try to get the theme from environment_variable
            let current = Theme::from_str(theme.as_str()).unwrap();
            match current {
                Theme::Dark => Theme::Light,
                Theme::Light => Theme::Dark,
            }
        }
        Err(e) => {
            // otherwise get the current from time of day
            get_theme_from_schedule(config)
        }
    }
}
