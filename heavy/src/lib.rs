use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::{env, fs, collections::HashMap};
use chrono::{Local, DateTime};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// show current and upcoming routines
    Show,

    /// mark routine as finished (works with future routines too)
    Finish {
        /// value of routine's "id" field
        id: String,
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub routines: Vec<Routine>,
}

#[derive(Debug, Deserialize)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub period: Option<String>,
}

pub fn read_config() -> Schedule {
    let config_path = format!("{}/.config/looper/schedule.toml", env::var("HOME").unwrap());
    toml::from_str(
        fs::read_to_string(&config_path)
            .unwrap_or_else(|_| panic!("No configuration file at {}", &config_path))
            .as_str()
    ).expect("Wrong configuration file format")
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub finish_times: HashMap<String, DateTime<Local>>,
}

// pub fn read_state() -> State {
//     let config_path = format!("{}/.config/looper/schedule.toml", env::var("HOME").unwrap());
//     toml::from_str(
//         fs::read_to_string(&config_path)
//             .unwrap_or_else(|_| panic!("No configuration file at {}", &config_path))
//             .as_str()
//     ).expect("Wrong configuration file format")
// }

pub fn write_state(state: &State) {
    let config_path = format!("{}/.config/looper/state.toml", env::var("HOME").unwrap());
    fs::write(&config_path, toml::to_string_pretty(state).unwrap()).unwrap();
}
