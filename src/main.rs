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

use clap::Parser;
use log;

fn main() {
    env_logger::init();
    // parse config
    let config_path = dirs::config_dir().unwrap().join("kswitch/config.toml");
    let config = match config_path.is_file() {
        true => Config::load(&config_path),
        false => {
            log::info!(
                "Creating default config at {}",
                config_path.to_string_lossy()
            );
            let config = Config::default();
            _ = config.save();
            Ok(config)
        }
    };

    match config {
        Err(e) => {
            log::error!(
                "Invalid config file at {}: {}",
                config_path.to_string_lossy(),
                e
            )
        }
        Ok(config) => {
            let cli = Cli::parse();

            match cli.command {
                Commands::Set { theme } => {
                    log::info!("Setting theme to {}", theme.to_string());
                    match theme {
                        Theme::Light => {
                            set(&theme, &config);
                        }
                        Theme::Dark => {
                            set(&theme, &config);
                        }
                    }
                }
                Commands::Config { command } => match command {
                    cli::ConfigCommand::List => {
                        log::info!("Listing configuration");
                        println!("{}", toml::to_string(&config).unwrap());
                    }
                    cli::ConfigCommand::Edit => {
                        log::info!("Opening configuration editor");
                        let _ = config.edit();
                    }
                },
                Commands::Toggle => {
                    log::info!("Toggling theme");
                    toggle(&config);
                }
            }
        }
    }
}
