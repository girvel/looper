use chrono::{DateTime, Local};
use colored::Colorize;
use cron::Schedule;
use std::{collections::HashMap, str::FromStr};
use heavy::{parse_cli, read_schedule, read_state, write_state, Commands, State};

fn main() {
    let cli = parse_cli();
    let schedule = read_schedule();
    let mut state = read_state();

    match cli.command {
        Commands::Show => {
            println!("{}", "Upcoming:".bright_white());
            for routine in schedule.routines {
                let Some(ref period) = routine.period else { continue };
                let cron_schedule = Schedule::from_str(&period).unwrap();
                // TODO coloring
                println!(
                    "{}  {}  {}",
                    format!("#{}", routine.id).bright_black(),
                    routine.name,
                    format!("@{}", cron_schedule.upcoming(Local).next().unwrap())
                        .bright_black(),
                );
            }
        },

        Commands::Finish { id } => {
            let period = &schedule.routines.iter()
                .filter(|r| r.id == id)
                .next()
                .unwrap_or_else(|| panic!("Unable to find a task with id {}", id))
                .period.as_ref()
                .unwrap_or_else(|| panic!("No period specified for task with id {}", id));

            state.finish_times.insert(
                id.clone(),
                Schedule::from_str(period)
                    .unwrap()
                    .after(state.finish_times.get(&id).unwrap_or(&Local::now()))
                    .next()
                    .unwrap(),
            );
            write_state(&state);
        },
    }
}
