use clap::{Error, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Theme types, also used as part of the cli subcommand structure
#[derive(Subcommand, Debug, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    #[command(about = "Set theme to Light")]
    Light,
    #[command(about = "Set theme to Dark")]
    Dark,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Style {
    pub wallpaper: PathBuf,
    pub color_scheme: String,
    pub desktop_theme: String,
    pub terminal_profile: String,
}
