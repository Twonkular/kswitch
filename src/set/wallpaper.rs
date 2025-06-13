use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, Output};

pub fn set(wallpaper: &PathBuf) -> Result<Output, Error> {
    // QDBUS alternative
    // qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "  ✔  base   !1/2 
    // var Desktops = desktops();
    // for (i = 0; i < Desktops.length; i++) {
    //   d = Desktops[i];
    //   d.wallpaperPlugin = 'org.kde.image';
    //   d.currentConfigGroup = Array('Wallpaper', 'org.kde.image', 'General');
    //   d.writeConfig('Image', 'file:///home/finley/Pictures/wallpapers/flower_mountain_scene/wp12238930-ai-4k-wallpapers.png');
    // }"

    println!("Setting wallpaper to  {}", &wallpaper.to_string_lossy());
    let out = Command::new("plasma-apply-wallpaperimage")
        .arg(wallpaper)
        .output();
    out
}

#[cfg(test)] // only build for tests
fn get_current_wallpaper() -> Option<PathBuf> {
    let config_path: PathBuf =
        dirs::home_dir()?.join(".config/plasma-org.kde.plasma.desktop-appletsrc");
    let contents = fs::read_to_string(config_path).ok()?;

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
