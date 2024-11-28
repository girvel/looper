use clap::{Parser, Subcommand, ValueEnum};
use serde::{Serialize, Deserialize};
use std::{env, fs, collections::HashMap};
use chrono::{Local, DateTime};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
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

    /// mark routine as finished (works with future routines too)
    Done {
        /// value of routine's "id" field
        ids: Vec<String>,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConfigType {
    Schedule,
    State,
}

impl ConfigType {
    pub fn get_path(&self) -> Result<String, String> {
        Ok(format!(
            "{}/.config/looper/{}.toml",
            env::var("HOME").map_err(|_| "Environment variable $HOME not set")?,
            match self {
                ConfigType::Schedule => { "schedule" },
                ConfigType::State => { "state" },
            },
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct Routine {
    pub name: String,
    pub period: String,
}

pub fn read_schedule() -> Result<HashMap<String, Routine>, String> {
    let path = ConfigType::Schedule.get_path()?;

    let grouped: HashMap<String, HashMap<String, String>> = toml::from_str(
        fs::read_to_string(&path)
            .map_err(|_| format!("No configuration file at {}", &path))?
            .as_str()
    ).map_err(|_| format!("Wrong schedule file format at {}", &path))?;

    Ok(grouped.iter()
        .flat_map(|(_, ids)| {
            let period = &ids["period"];
            ids.iter()
                .filter(|(id, _)| *id != "period")
                .map(|(id, name)| (id.clone(), Routine {
                    name: name.clone(),
                    period: period.clone(),
                }))
        })
        .collect())
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub finish_times: HashMap<String, DateTime<Local>>,
}

// TODO probably should be like ConfigType.read
// TODO probably should have separate Cli & Data files
pub fn read_state() -> Result<State, String> {
    let path = ConfigType::State.get_path()?;
    let Ok(content) = fs::read_to_string(path)
        else { return Ok(State { finish_times: HashMap::new(), }) };
    Ok(toml::from_str(content.as_str()).expect("Wrong state file format"))
}

pub fn write_state(state: &State) -> Result<(), String> {
    let path = ConfigType::State.get_path()?;
    fs::write(path, toml::to_string_pretty(state).unwrap()).unwrap();
    Ok(())
}
