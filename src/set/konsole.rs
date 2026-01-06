use crate::{config::Config, theme::Theme};
use log;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::result::Result;
use zbus;
use zbus::blocking::{Connection, Proxy};

fn set_default_profile(theme: &Theme, config: &Config) -> Result<(), Box<dyn Error>> {
    log::debug!(
        "Setting default Konsole profile to: {}.profile",
        theme.to_string()
    );

    // Read the file contents into a String
    let contents = fs::read_to_string(&config.konsolerc)?;

    // Prepare a buffer to store the modified lines
    let mut output = Vec::new();
    let mut in_desktop_entry = false;

    for line in contents.lines() {
        let trimmed = line.trim();

        // Track section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_desktop_entry = trimmed == "[Desktop Entry]";
        }

        // Update the DefaultProfile key if in the right section
        if in_desktop_entry && trimmed.starts_with("DefaultProfile=") {
            output.push(format!("DefaultProfile={}.profile", theme.to_string()).to_string());
        } else {
            output.push(line.to_string());
        }
    }

    // Write the modified content back to the file
    let mut file = fs::File::create(&config.konsolerc)?;
    for line in output {
        writeln!(file, "{}", line)?;
    }

    log::info!(
        "Default Konsole profile updated to: {}.profile",
        theme.to_string()
    );
    Ok(())
}

fn set_session_theme(session_id: &String, theme: &Theme) -> Result<(), Box<dyn Error>> {
    log::debug!(
        "Setting Konsole session {} profile to: {}",
        session_id,
        theme.to_string()
    );

    // Connect to the session bus
    let connection = Connection::session()?;

    // Compose the D-Bus destination and object path
    let service_name = format!("{}", session_id);
    let object_path = "/Sessions/1"; // Hardcoded like your qdbus example

    // Create a proxy for the org.kde.konsole.Session interface
    let proxy = Proxy::new(
        &connection,
        service_name.as_str(),
        object_path,
        "org.kde.konsole.Session",
    )?;

    // Call the setProfile method
    proxy.call_method("setProfile", &theme.to_string())?;

    log::info!(
        "Konsole session {} profile set to: {}",
        session_id,
        theme.to_string()
    );
    Ok(())
}

fn get_session_ids() -> zbus::Result<Vec<String>> {
    log::debug!("Retrieving list of Konsole session IDs from D-Bus");

    // Connect to the session bus
    let connection = Connection::session()?;

    // Get a proxy to the org.freedesktop.DBus interface
    let proxy = zbus::blocking::fdo::DBusProxy::new(&connection)?;

    // List all names registered on the bus
    let names = proxy.list_names()?;

    // Filter names that contain "org.kde.konsole"
    let konsole_names: Vec<String> = names
        .into_iter()
        .filter(|name| name.contains("org.kde.konsole"))
        .map(|name| name.to_string())
        .collect();

    log::debug!("Found {} Konsole session(s)", konsole_names.len());
    Ok(konsole_names)
}

pub fn set(theme: &Theme, config: &Config) {
    log::info!("Configuring Konsole theme to: {}", theme.to_string());

    // Set the default profile in konsolerc
    match set_default_profile(theme, config) {
        Ok(_) => {
            log::debug!("Successfully updated konsolerc default profile");
        }
        Err(e) => {
            log::warn!("Failed to update default Konsole profile: {}", e);
        }
    }

    // Get all active Konsole sessions and update their profiles
    match get_session_ids() {
        Ok(session_ids) => {
            if session_ids.is_empty() {
                log::debug!("No active Konsole sessions found");
                return;
            }

            log::info!(
                "Updating {} active Konsole session(s) to theme: {}",
                session_ids.len(),
                theme.to_string()
            );

            for session_id in session_ids.iter() {
                match set_session_theme(session_id, theme) {
                    Ok(_) => {
                        log::debug!("Session {} profile updated successfully", session_id);
                    }
                    Err(e) => {
                        log::warn!("Failed to update session {} profile: {}", session_id, e);
                    }
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to retrieve Konsole session IDs: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_open_konsoles() {
        let konsole_ids = get_session_ids().unwrap();

        for id in konsole_ids.iter() {
            println!("{}", id);
        }
    }
}
