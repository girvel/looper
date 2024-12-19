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
    /// show current state of the schedule
    Show {
        /// whether to display the long version
        #[arg(short, long)]
        verbose: bool,
    },

    /// mark routine as finished (works with upcoming routines too)
    Done {
        /// value of routine's "id" field
        ids: Vec<String>,
    },

    /// get path for given config
    Path {
        /// the type of the config
        config_type: ConfigType,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
