use clap::{Error, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

/// Theme types, also used as part of the cli subcommand structure
#[derive(Subcommand, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Theme {
    #[command(about = "Set theme to Light")]
    Light,
    #[command(about = "Set theme to Dark")]
    Dark,
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
        }
    }
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Style {
    pub wallpaper: PathBuf,
    pub color_scheme: String,
    pub desktop_theme: String,
    pub terminal_profile: String,
}
