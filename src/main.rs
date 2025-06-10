mod cli;
mod config;
mod operations;
mod set;
mod theme;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::operations::set;
use crate::theme::Theme;

fn main() {
    // parse config
    let config_path = dirs::config_dir().unwrap().join("kswitch/config.toml");
    let config = match config_path.is_file() {
        true => {
            println!("{}", &config_path.to_string_lossy());
            Config::load(&config_path)
        }
        false => {
            let config = Config::default();
            config.save();
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
                Commands::Toggle => {
                    println!("Performing Toggle!");
                }
                Commands::Set { theme } => match theme {
                    Theme::Light => {
                        dbg!("Setting Light theme");
                        set(&config.light);
                    }
                    Theme::Dark => {
                        dbg!("Setting Dark theme");
                        set(&config.dark);
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
            }
        }
    }
}
