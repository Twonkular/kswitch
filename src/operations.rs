use crate::config::Config;
use crate::set::{color_scheme, global_theme, konsole, wallpaper};
use crate::state::StateManager;
use crate::theme::Theme;
use std::process::Command;
use std::sync::{Arc, Barrier};
use std::thread;

use log;

use crate::get::target_theme;

pub fn set(theme: &Theme, config: &Config) {
    log::info!("Setting theme to {}", theme.to_string());

    let style = match theme {
        Theme::Dark => &config.dark,
        Theme::Light => &config.light,
    };

    let global_theme = style.desktop_theme.clone();
    let wallpaper = style.wallpaper.clone();
    let color_scheme = style.color_scheme.clone();

    log::debug!(
        "Applying theme settings: desktop_theme={}, color_scheme={}",
        global_theme,
        color_scheme
    );

    // Barrier for synchronizing thread start
    let barrier = Arc::new(Barrier::new(4)); // 3 worker threads + main

    let barrier1 = Arc::clone(&barrier);
    let theme_handle = thread::spawn(move || {
        barrier1.wait(); // Wait until all threads are ready
        log::debug!("Applying desktop theme: {}", global_theme);
        let _ = global_theme::set(&global_theme);
    });

    let barrier2 = Arc::clone(&barrier);
    let wallpaper_handle = thread::spawn(move || {
        barrier2.wait(); // Wait until all threads are ready
        log::debug!("Applying wallpaper: {}", wallpaper.to_string_lossy());
        let _ = wallpaper::set(&wallpaper);
    });

    let barrier3 = Arc::clone(&barrier);
    let color_scheme_handle = thread::spawn(move || {
        barrier3.wait(); // Wait until all threads are ready
        log::debug!("Applying color scheme: {}", color_scheme);
        let _ = color_scheme::set(&color_scheme);
    });

    barrier.wait(); // Let the threads go at the same time

    // Wait for all threads to complete
    let _ = theme_handle.join();
    let _ = wallpaper_handle.join();
    let _ = color_scheme_handle.join();

    // apply default theme to konsole
    // This does not need to be done in parallel as it is non-visual
    log::debug!("Applying Konsole profile for theme: {}", theme.to_string());
    konsole::set(&theme, &config);

    // Save the theme state to file
    log::debug!("Saving theme state: {}", theme.to_string());
    match StateManager::new() {
        Ok(state_manager) => match state_manager.save(theme) {
            Ok(_) => {
                log::debug!("Theme state persisted successfully");
            }
            Err(e) => {
                log::warn!("Failed to save theme state: {}", e);
            }
        },
        Err(e) => {
            log::warn!("Failed to initialize state manager for saving: {}", e);
        }
    }

    // Run user scripts for the theme
    run_theme_scripts(theme, config);

    log::info!("Theme successfully set to {}", theme.to_string());
}

fn run_theme_scripts(theme: &Theme, config: &Config) {
    let scripts_dir = match theme {
        Theme::Dark => &config.dark_scripts_dir,
        Theme::Light => &config.light_scripts_dir,
    };

    log::debug!(
        "Looking for {} theme scripts in {}",
        theme.to_string(),
        scripts_dir.to_string_lossy()
    );

    // Check if directory exists
    if !scripts_dir.is_dir() {
        log::debug!(
            "Scripts directory does not exist: {}",
            scripts_dir.to_string_lossy()
        );
        return;
    }

    // Read all files in the scripts directory
    match std::fs::read_dir(scripts_dir) {
        Ok(entries) => {
            let script_files: Vec<_> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_file() { Some(path) } else { None }
                    })
                })
                .collect();

            if script_files.is_empty() {
                log::debug!(
                    "No scripts found in directory: {}",
                    scripts_dir.to_string_lossy()
                );
                return;
            }

            log::info!("Found {} theme script(s) to execute", script_files.len());

            for path in script_files {
                log::info!("Running script: {}", path.to_string_lossy());
                match Command::new(&path).status() {
                    Ok(status) => {
                        if status.success() {
                            log::info!("Script executed successfully: {}", path.to_string_lossy());
                        } else {
                            log::warn!(
                                "Script exited with non-zero status: {}",
                                path.to_string_lossy()
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to execute script {}: {}", path.to_string_lossy(), e);
                    }
                }
            }
        }
        Err(e) => {
            log::error!(
                "Failed to read scripts directory {}: {}",
                scripts_dir.to_string_lossy(),
                e
            );
        }
    }
}

pub fn toggle(config: &Config) {
    log::info!("Toggling theme");
    let target_theme = target_theme::get(config);

    log::debug!(
        "Current theme is {}, switching to {}",
        if target_theme == Theme::Dark {
            "Light"
        } else {
            "Dark"
        },
        target_theme.to_string()
    );

    // set to target theme
    set(&target_theme, &config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_light() {
        let config = Config::default();
        let theme = Theme::Light;
        set(&theme, &config);
    }

    #[test]
    fn test_set_dark() {
        let config = Config::default();
        let theme = Theme::Dark;
        set(&theme, &config);
    }

    #[test]
    fn test_toggle() {
        let config = Config::default();
        toggle(&config);
    }
}
