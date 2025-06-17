use dirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use toml;

use crate::schedule::{self, Schedule};
use crate::theme::Style;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(skip)]
    pub path: PathBuf,
    pub light: Style,
    pub dark: Style,
    pub schedule: Schedule,
    pub konsolerc: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let path = dirs::config_dir().unwrap().join("kswitch/config.toml");

        if path.is_file() {
            Config::load(&path).unwrap()
        } else {
            let light_style = Style {
                wallpaper: PathBuf::from(
                    "/usr/share/wallpapers/Bamboo/contents/images/5120x2880.png",
                ),
                color_scheme: String::from("BreathLight"),
                desktop_theme: String::from("breath"),
                terminal_profile: String::from("light"),
            };
            let dark_style = Style {
                wallpaper: PathBuf::from(
                    "/usr/share/wallpapers/Bamboo at Night/contents/images/5120x2880.png",
                ),
                color_scheme: String::from("BreathDark"),
                desktop_theme: String::from("breath-dark"),
                terminal_profile: String::from("dark"),
            };
            let schedule = Schedule::default();
            Config {
                path: path,
                light: light_style,
                dark: dark_style,
                schedule: schedule,
                konsolerc: PathBuf::from(
                    dirs::config_dir()
                        .unwrap_or(PathBuf::from("~/.config"))
                        .join("konsolerc"),
                ),
            }
        }
    }
}

impl Config {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        // Serialize the config struct to a TOML string
        let toml_string = toml::to_string(&self)?;

        // create config dir if needed
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent).unwrap();
        }

        // Open a file in write mode
        let mut file = File::create(&self.path)?;

        // Write the serialized string to the file
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    pub fn edit(&self) -> Result<(), Box<dyn Error>> {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        // spawn the editor process
        let mut command = Command::new(editor);
        println!("PATH {}", self.path.to_string_lossy());
        command.arg(&self.path);
        println!("{}", self.path.to_string_lossy());

        // execute command
        match command.spawn() {
            Ok(mut child) => match child.wait() {
                Ok(status) => println!("Editor exited with status: {}", status),
                Err(e) => eprintln!("Failed to wait on editor process: {}", e),
            },
            Err(e) => eprintln!("Failed to start editor: {}", e),
        };
        Ok(())
    }

    pub fn load(file_path: &PathBuf) -> Result<Config, Box<dyn Error>> {
        // Open the file in read mode
        let mut file = File::open(file_path)?;

        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the TOML string into a Config struct
        let mut config: Config = toml::from_str(&contents)?;
        config.path = dirs::config_dir().unwrap().join("kswitch/config.toml");

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use env::temp_dir;
    use std::fs::remove_file;

    use super::*;

    #[test]
    fn test_create_and_load_config() {
        let light_style = Style {
            wallpaper: PathBuf::from("/usr/share/wallpapers/Bamboo/contents/images/5120x2880.png"),
            color_scheme: String::from("BreathLight"),
            desktop_theme: String::from("breath"),
            terminal_profile: String::from("light"),
        };
        let dark_style = Style {
            wallpaper: PathBuf::from(
                "/usr/share/wallpapers/Bamboo at Night/contents/images/5120x2880.png",
            ),
            color_scheme: String::from("BreathDark"),
            desktop_theme: String::from("breath-dark"),
            terminal_profile: String::from("light"),
        };
        let conf = Config {
            path: temp_dir().join("testconfig.toml"),
            light: light_style,
            dark: dark_style,
            schedule: Schedule::default(),
            konsolerc: PathBuf::from(
                dirs::config_dir()
                    .unwrap_or(PathBuf::from("~/.config"))
                    .join("konsolerc"),
            ),
        };

        let _ = conf.save();

        let loaded = Config::load(&conf.path).unwrap();
        assert!(loaded.light.color_scheme == String::from("BreathLight"));
        assert!(loaded.dark.color_scheme == String::from("BreathDark"));
        assert!(loaded.konsolerc.is_file());

        let _ = remove_file(loaded.path);
    }
}
