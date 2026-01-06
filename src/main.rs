mod cli;
mod config;
mod get;
mod operations;
mod schedule;
mod set;
mod state;
mod theme;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::operations::{set, toggle};
use crate::theme::Theme;

use clap::Parser;
use log;

fn main() {
    env_logger::init();

    log::debug!("kswitch starting");

    // parse config
    let config_path = dirs::config_dir().unwrap().join("kswitch/config.toml");
    let config = match config_path.is_file() {
        true => {
            log::info!("Loading config from {}", config_path.to_string_lossy());
            Config::load(&config_path)
        }
        false => {
            log::info!(
                "Config not found, creating default config at {}",
                config_path.to_string_lossy()
            );
            let config = Config::default();
            if let Err(e) = config.save() {
                log::error!("Failed to save default config: {}", e);
            }
            Ok(config)
        }
    };

    match config {
        Err(e) => {
            log::error!(
                "Failed to load config from {}: {}",
                config_path.to_string_lossy(),
                e
            );
            println!(
                "Error:\tInvalid config file at {}",
                config_path.to_string_lossy()
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
                        log::debug!("Listing config");
                        println!("{}", toml::to_string(&config).unwrap());
                    }
                    cli::ConfigCommand::Edit => {
                        log::info!("Opening config for editing");
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

    log::debug!("kswitch finished");
}
