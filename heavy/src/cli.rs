use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::config::ConfigType;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, value_name = "FOLDER")]
    pub config_folder: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show current state of the schedule
    Show {
        /// Whether to display the long version
        #[arg(short, long)]
        verbose: bool,
    },

    /// Mark routine as finished (works with upcoming routines too)
    Done {
        /// Routine's unique ID
        ids: Vec<String>,
    },

    /// Get path for given config
    Path {
        /// Type of the config
        config_type: ConfigType,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
