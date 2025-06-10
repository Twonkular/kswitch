use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::theme::Theme;

/// Set of possible commands to use with the cli interface
#[derive(Subcommand, Debug, Serialize)]
pub enum Commands {
    #[command(
        about = "Set theme to either Dark or Light",
        arg_required_else_help = true,
        after_help = "
\x1b[1mExample usage:\x1b[0m
    kswitch set light
    kswitch set dark"
    )]
    Set {
        #[command(subcommand)]
        theme: Theme,
    },
    #[command(about = "Toggle the theme between Light and Dark")]
    Toggle,
    #[command(about = "Configure for kswitch", arg_required_else_help = true)]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

/// kswitch: theme switching tool for KDE Plasma
#[derive(Parser, Serialize, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Serialize, Deserialize, PartialEq)]
pub enum ConfigCommand {
    List,
    Edit,
}
