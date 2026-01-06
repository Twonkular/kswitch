use log;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

use zbus::Result as ZbusResult;
use zbus::blocking::Connection;

pub fn set(wallpaper: &PathBuf) -> Result<Output, Error> {
    log::info!("Applying wallpaper: {}", wallpaper.to_string_lossy());

    // Convert the path to a file:// URI
    let wallpaper_uri = format!("file://{}", wallpaper.display());

    // JavaScript script sent to plasmashell via D-Bus
    let script = format!(
        "var Desktops = desktops();
         for (i = 0; i < Desktops.length; i++) {{
             d = Desktops[i];
             d.wallpaperPlugin = 'org.kde.image';
             d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
             d.writeConfig('Image', '{}');
         }}",
        wallpaper_uri
    );

    log::debug!("Sending wallpaper configuration via D-Bus to plasmashell");

    // Call D-Bus
    match send_dbus_script(&script) {
        Ok(_) => {
            log::info!(
                "Wallpaper applied successfully: {}",
                wallpaper.to_string_lossy()
            );
            Ok(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        }
        Err(e) => {
            log::error!("Failed to apply wallpaper via D-Bus: {}", e);
            Err(Error::new(
                std::io::ErrorKind::Other,
                format!("D-Bus error: {e}"),
            ))
        }
    }
}

// Helper to send the D-Bus message
fn send_dbus_script(script: &str) -> ZbusResult<()> {
    log::debug!("Connecting to D-Bus session");
    let connection = Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &connection,
        "org.kde.plasmashell",
        "/PlasmaShell",
        "org.kde.PlasmaShell",
    )?;

    log::debug!("Calling plasmashell evaluateScript method");
    proxy.call_method("evaluateScript", &(script))?;
    log::debug!("D-Bus script execution completed");
    Ok(())
}

#[cfg(test)] // only build for tests
fn get_current_wallpaper() -> Option<PathBuf> {
    let config_path: PathBuf =
        dirs::home_dir()?.join(".config/plasma-org.kde.plasma.desktop-appletsrc");
    let contents = std::fs::read_to_string(config_path).ok()?;

    let mut in_wallpaper_section = false;
    for line in contents.lines() {
        if line.starts_with("[Containments]") && line.contains("Wallpaper][org.kde.image][General]")
        {
            in_wallpaper_section = true;
        } else if line.starts_with('[') {
            in_wallpaper_section = false;
        } else if in_wallpaper_section && line.starts_with("Image=") {
            // Extract the path after "Image="
            let image_path = line.trim_start_matches("Image=").trim();
            // Remove "file://" prefix if present
            let local_path = image_path.strip_prefix("file://").unwrap_or(image_path);
            return Some(PathBuf::from(local_path.to_string()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_set_wallpaper() {
        let config = Config::default();

        let current = get_current_wallpaper().expect("Failed to get current wallpaper");

        let out = if current == config.light.wallpaper {
            set(&config.dark.wallpaper)
        } else {
            set(&config.light.wallpaper)
        };

        assert!(out.is_ok());
    }
}
