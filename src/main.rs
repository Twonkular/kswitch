mod cli;
mod theme;

use clap::{Command, Parser};

use cli::{Cli, Commands};
use theme::Theme;

fn main() {
    let mut cli = Cli::parse();

    match cli.command {
        Some(ref mut com) => match com {
            Commands::Toggle => {
                println!("Performing Toggle!");
            }
            Commands::Set { theme } => match theme {
                Theme::Light => {
                    println!("Setting Light theme");
                }
                Theme::Dark => {
                    println!("Setting Dark theme");
                }
            },
        },
        None => {}
    }
}
