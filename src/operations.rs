use crate::config::Config;
use crate::set::{global_theme, wallpaper};
use crate::theme::Style;
use std::thread;

pub fn set(style: &Style) {
    // TODO: remove need to clone here?
    let theme = style.desktop_theme.clone();
    let wallpaper = style.wallpaper.clone();

    let theme_handle = thread::spawn(move || {
        let _ = global_theme::set(&theme);
    });

    let wallpaper_handle = thread::spawn(move || {
        let _ = wallpaper::set(&wallpaper);
    });

    // Wait for both to finish
    let _ = theme_handle.join();
    let _ = wallpaper_handle.join();
}

pub fn toggle(config: Config) {
    todo!();
}
