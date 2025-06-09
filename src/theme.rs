use clap::Subcommand;
use serde::Serialize;

/// Theme types, also used as part of the cli subcommand structure
#[derive(Subcommand, Debug, Serialize)]
pub enum Theme {
    #[command(about = "Set theme to Light")]
    Light,
    #[command(about = "Set theme to Dark")]
    Dark,
}
