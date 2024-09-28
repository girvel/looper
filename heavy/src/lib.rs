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
    Done {
        /// value of routine's "id" field
        id: String,
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[derive(Debug, Deserialize)]
pub struct Routine {
    pub name: String,
    pub period: Option<String>,
}

pub fn read_schedule() -> HashMap<String, Routine> {
    let path = format!("{}/.config/looper/schedule.toml", env::var("HOME").unwrap());
    toml::from_str(
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("No configuration file at {}", &path))
            .as_str()
    ).expect("Wrong schedule file format")
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub finish_times: HashMap<String, DateTime<Local>>,
}

pub fn read_state() -> State {
    let path = format!("{}/.config/looper/state.toml", env::var("HOME").unwrap());
    let Ok(content) = fs::read_to_string(&path)
        else { return State { finish_times: HashMap::new(), }};
    toml::from_str(content.as_str()).expect("Wrong state file format")
}

pub fn write_state(state: &State) {
    let path = format!("{}/.config/looper/state.toml", env::var("HOME").unwrap());
    fs::write(&path, toml::to_string_pretty(state).unwrap()).unwrap();
}
