use clap::{Parser, Subcommand, ValueEnum};
use serde::{Serialize, Deserialize};
use std::{env, fs, collections::HashMap};
use chrono::{Local, DateTime};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConfigType {
    Schedule,
    State,
}

impl ConfigType {
    pub fn get_path(&self) -> String {
        format!(
            "{}/.config/looper/{}.toml",
            env::var("HOME").unwrap(),
            match self {
                ConfigType::Schedule => { "schedule" },
                ConfigType::State => { "state" },
            },
        )
    }
}

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

    /// get path for given config
    Path {
        /// the type of the config
        config_type: ConfigType,
    }
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[derive(Debug, Deserialize)]
pub struct Routine {
    pub name: String,
    pub period: String,
}

pub fn read_schedule() -> HashMap<String, Routine> {
    let path = ConfigType::Schedule.get_path();

    let grouped: HashMap<String, HashMap<String, String>> = toml::from_str(
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("No configuration file at {}", &path))
            .as_str()
    ).expect("Wrong schedule file format");

    grouped.iter()
        .flat_map(|(_, ids)| {
            let period = &ids["period"];
            ids.iter()
                .filter(|(id, _)| *id != "period")
                .map(|(id, name)| (id.clone(), Routine {
                    name: name.clone(),
                    period: period.clone(),
                }))
        })
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub finish_times: HashMap<String, DateTime<Local>>,
}

pub fn read_state() -> State {
    let path = ConfigType::State.get_path();
    let Ok(content) = fs::read_to_string(path)
        else { return State { finish_times: HashMap::new(), }};
    toml::from_str(content.as_str()).expect("Wrong state file format")
}

pub fn write_state(state: &State) {
    let path = format!("{}/.config/looper/state.toml", env::var("HOME").unwrap());
    fs::write(path, toml::to_string_pretty(state).unwrap()).unwrap();
}
