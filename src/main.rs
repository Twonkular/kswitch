mod cli;
mod config;
mod get;
mod operations;
mod schedule;
mod set;
mod theme;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::operations::{set, toggle};
use crate::theme::Theme;

use clap::{Parser, command};

fn main() {
    // parse config
    let config_path = dirs::config_dir().unwrap().join("kswitch/config.toml");
    let config = match config_path.is_file() {
        true => Config::load(&config_path),
        false => {
            let config = Config::default();
            _ = config.save();
            Ok(config)
        }
    };

    match config {
        Err(e) => {
            println!(
                "Error:\tInvalid config file at {}",
                config_path.to_string_lossy()
            )
        }
        Ok(config) => {
            let cli = Cli::parse();

            match cli.command {
                Commands::Set { theme } => match theme {
                    Theme::Light => {
                        dbg!("Setting Light theme");
                        set(&config.light, &theme, &config);
                    }
                    Theme::Dark => {
                        dbg!("Setting Dark theme");
                        set(&config.dark, &theme, &config);
                    }
                },
                Commands::Config { command } => match command {
                    cli::ConfigCommand::List => {
                        println!("{}", toml::to_string(&config).unwrap());
                    }
                    cli::ConfigCommand::Edit => {
                        let _ = config.edit();
                    }
                },
                Commands::Toggle => {
                    toggle(&config);
                }
            }
        }
    }
}
