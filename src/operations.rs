use crate::config::Config;
use crate::set::{global_theme, wallpaper};
use crate::theme::Style;

pub fn set(style: &Style) {
    let _ = global_theme::set(&style.desktop_theme);
    let _ = wallpaper::set(&style.wallpaper);
}

pub fn toggle(config: Config) {
    todo!();
}
