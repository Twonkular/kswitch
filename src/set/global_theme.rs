use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, Output};

pub fn set(theme_name: &String) -> Result<Output, Error> {
    let out = Command::new("plasma-apply-desktoptheme")
        .arg(theme_name)
        .output();
    out
}

/// Returns the current KDE Plasma desktop theme name, if found.
fn get_current_desktop_theme() -> Option<String> {
    let config_path: PathBuf = dirs::home_dir()?.join(".config/plasmarc");
    let contents = fs::read_to_string(config_path).ok()?;

    let mut in_theme_section = false;
    for line in contents.lines() {
        if line.trim() == "[Theme]" {
            in_theme_section = true;
        } else if line.starts_with('[') {
            in_theme_section = false;
        } else if in_theme_section && line.starts_with("name=") {
            let theme_name = line.trim_start_matches("name=").trim();
            return Some(theme_name.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_set_desktop_theme() {
        let config = Config::default();

        let current = get_current_desktop_theme().expect("Failed to get current wallpaper");

        let out = if current == config.light.desktop_theme {
            set(&config.dark.desktop_theme)
        } else {
            set(&config.light.desktop_theme)
        };

        assert!(out.is_ok());
    }
}
