use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{env, fs};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Show {},
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub routines: Vec<Routine>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub period: String,
}

pub fn read_config() -> Schedule {
    let config_path = format!("{}/.config/looper/schedule.toml", env::var("HOME").unwrap());
    toml::from_str(
        fs::read_to_string(&config_path)
            .unwrap_or_else(|_| panic!("No configuration file at {}", &config_path))
            .as_str()
    ).expect("Wrong configuration file format")
}
