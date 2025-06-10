use crate::config::Config;
use crate::set::wallpaper;
use crate::theme::Style;

pub fn set(style: &Style) {
    let _ = wallpaper::set(&style.wallpaper);
}

pub fn toggle(config: Config) {
    todo!();
}
