use serde::{Serialize, Deserialize};
use std::{collections::HashMap, env, fs, path::PathBuf};
use chrono::{Local, DateTime};
use clap::ValueEnum;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConfigType {
    Schedule,
    State,
}

impl ConfigType {
    pub fn get_path(&self, config_folder: Option<&PathBuf>) -> Result<String, String> {
        let final_config_folder = match config_folder {
            Some(p) => p.to_str().unwrap().to_string(),
            None => format!(
                "{}/.config/looper",
                env::var("HOME").map_err(|_| "Environment variable $HOME not set")?
            ),
        };

        Ok(format!(
            "{}/{}.toml",
            final_config_folder,
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



pub type Schedule = HashMap<String, Routine>;

pub fn read_schedule(config_folder: Option<&PathBuf>) -> Result<Schedule, String> {
    let path = ConfigType::Schedule.get_path(config_folder)?;

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

pub type State = HashMap<String, DateTime<Local>>;

#[derive(Serialize, Deserialize)]
struct StateContainer {
    finish_times: State,
}

pub fn read_state(config_folder: Option<&PathBuf>) -> Result<State, String> {
    let path = ConfigType::State.get_path(config_folder)?;
    let Ok(content) = fs::read_to_string(path)
        else { return Ok(HashMap::new()) };
    Ok(toml::from_str::<StateContainer>(content.as_str()).expect("Wrong state file format").finish_times)
}

pub fn write_state(config_folder: Option<&PathBuf>, state: &State) -> Result<(), String> {
    let path = ConfigType::State.get_path(config_folder)?;
    fs::write(path, toml::to_string_pretty(state).unwrap()).unwrap();
    Ok(())
}
