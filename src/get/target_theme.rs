use crate::theme::Theme;
use chrono::Local;
use log;
use std::env;
use std::str::FromStr;

use crate::config::Config;

fn get_theme_from_schedule(config: &Config) -> Theme {
    let time = Local::now().naive_local().time();
    log::debug!(
        "Determining theme from schedule at time: {}",
        time.format("%H:%M:%S")
    );
    let theme = config.schedule.theme_from_time(&time);
    log::debug!("Schedule-based theme: {}", theme.to_string());
    theme
}

/// Gets the current kswitch theme, either by reading the environment variable, if it exists, otherwise it is determined from the time of day and the schedule defined in config.
pub fn get(config: &Config) -> Theme {
    log::debug!("Determining target theme for toggle");

    match env::var("KSWITCH_THEME") {
        Ok(theme_str) => {
            log::debug!("KSWITCH_THEME environment variable found: {}", theme_str);
            // Try to get the theme from environment_variable
            match Theme::from_str(theme_str.as_str()) {
                Ok(current) => {
                    let target = match current {
                        Theme::Dark => Theme::Light,
                        Theme::Light => Theme::Dark,
                    };
                    log::debug!(
                        "Current theme from env: {}, target theme: {}",
                        current.to_string(),
                        target.to_string()
                    );
                    target
                }
                Err(_) => {
                    log::warn!(
                        "Invalid KSWITCH_THEME value: {}, falling back to schedule",
                        theme_str
                    );
                    get_theme_from_schedule(config)
                }
            }
        }
        Err(_) => {
            log::debug!("KSWITCH_THEME environment variable not set, using schedule");
            // otherwise get the current from time of day
            get_theme_from_schedule(config)
        }
    }
}
