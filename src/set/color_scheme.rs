use log;
use std::io::Error;
use std::process::{Command, Output};

pub fn set(color_scheme_name: &String) -> Result<Output, Error> {
    log::info!("Applying color scheme: {}", color_scheme_name);
    let out = Command::new("plasma-apply-colorscheme")
        .arg(color_scheme_name)
        .output();

    match &out {
        Ok(output) if output.status.success() => {
            log::info!("Color scheme applied successfully: {}", color_scheme_name);
        }
        Ok(output) => {
            log::warn!(
                "Failed to apply color scheme {}: {:?}",
                color_scheme_name,
                output.status
            );
        }
        Err(e) => {
            log::error!("Error applying color scheme {}: {}", color_scheme_name, e);
        }
    }
    out
}

/// Returns the current KDE Plasma color scheme name, if found.
#[cfg(test)] // only build for tests
fn get_current_color_scheme() -> Option<String> {
    let config_path = dirs::home_dir()?.join(".config/kdeglobals");
    let contents = std::fs::read_to_string(config_path).ok()?;

    let mut in_general_section = false;
    for line in contents.lines() {
        if line.trim() == "[General]" {
            in_general_section = true;
        } else if line.starts_with('[') {
            in_general_section = false;
        } else if in_general_section && line.starts_with("ColorScheme=") {
            let scheme_name = line.trim_start_matches("ColorScheme=").trim();
            return Some(scheme_name.to_string());
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

        let current = get_current_color_scheme().expect("Failed to get current wallpaper");

        let out = if current == config.light.color_scheme {
            set(&config.dark.color_scheme)
        } else {
            set(&config.light.color_scheme)
        };

        assert!(out.is_ok());
    }
}
