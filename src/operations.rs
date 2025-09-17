use crate::config::Config;
use crate::set::{color_scheme, global_theme, konsole, wallpaper};
use crate::theme::{Style, Theme};
use std::env::set_var;
use std::process::Command;
use std::sync::{Arc, Barrier};
use std::thread;

use std::fs;

use crate::get::target_theme;

pub fn set(theme: &Theme, config: &Config) {
    let style = match theme {
        Theme::Dark => &config.dark,
        Theme::Light => &config.light,
    };

    let global_theme = style.desktop_theme.clone();
    let wallpaper = style.wallpaper.clone();
    let color_scheme = style.color_scheme.clone();

    // Barrier for synchronizing thread start
    let barrier = Arc::new(Barrier::new(4)); // 3 worker threads + main

    let barrier1 = Arc::clone(&barrier);
    let theme_handle = thread::spawn(move || {
        barrier1.wait(); // Wait until all threads are ready
        let _ = global_theme::set(&global_theme);
    });

    let barrier2 = Arc::clone(&barrier);
    let wallpaper_handle = thread::spawn(move || {
        barrier2.wait(); // Wait until all threads are ready
        let _ = wallpaper::set(&wallpaper);
    });

    let barrier3 = Arc::clone(&barrier);
    let color_scheme_handle = thread::spawn(move || {
        barrier3.wait(); // Wait until all threads are ready
        let _ = color_scheme::set(&color_scheme);
    });

    barrier.wait(); // Let the threads go at the same time

    // Wait for all threads to complete
    let _ = theme_handle.join();
    let _ = wallpaper_handle.join();
    let _ = color_scheme_handle.join();

    // apply default theme to konsole
    // This does not need to be done in parallel as it is non-visual
    konsole::set(&theme, &config);

    // set environment variable for theme
    let _ = Command::new("systemctl")
        .arg("--user")
        .arg("import-environment")
        .arg("KSWITCH_THEME")
        .env("KSWITCH_THEME", &theme.to_string().as_str()) // pass clone or reference
        .status();

    // set environment variable for current session
    unsafe {
        let _ = set_var("KSWITCH_THEME", &theme.to_string().as_str());
    }
}

pub fn toggle(config: &Config) {
    let target_theme = target_theme::get(config);

    // set to non-current state
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
