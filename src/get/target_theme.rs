use crate::config::Config;
use crate::state::StateManager;
use crate::theme::Theme;
use chrono::Local;
use log;

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

/// Gets the target theme for toggling by reading the saved state file.
/// If the state file doesn't exist, falls back to schedule-based theme determination.
pub fn get(config: &Config) -> Theme {
    log::debug!("Determining target theme for toggle");

    match StateManager::new() {
        Ok(state_manager) => {
            match state_manager.load() {
                Ok(state) => {
                    log::debug!(
                        "Current theme from state file: {}",
                        state.current_theme.to_string()
                    );
                    // Return the opposite theme for toggle
                    let target = match state.current_theme {
                        Theme::Dark => Theme::Light,
                        Theme::Light => Theme::Dark,
                    };
                    log::debug!(
                        "Current theme: {}, target theme: {}",
                        state.current_theme.to_string(),
                        target.to_string()
                    );
                    target
                }
                Err(e) => {
                    log::warn!(
                        "Failed to load theme state: {}, falling back to schedule",
                        e
                    );
                    get_theme_from_schedule(config)
                }
            }
        }
        Err(e) => {
            log::error!("Failed to initialize state manager: {}, using schedule", e);
            get_theme_from_schedule(config)
        }
    }
}
