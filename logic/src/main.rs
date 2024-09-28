use colored::Colorize;
use cron::Schedule;
use std::{collections::HashMap, str::FromStr};
use heavy::{parse_cli, read_config, State, Commands};

fn main() {
    let cli = parse_cli();
    let schedule = read_config();
    let _state = State { finish_times: HashMap::new(), };

    match cli.command {
        Commands::Show => {
            println!("{}", "Upcoming:".bright_white());
            for routine in schedule.routines {
                let Some(period) = routine.period else { continue };
                let cron_schedule = Schedule::from_str(&period).unwrap();
                // TODO coloring
                println!(
                    "{}  {}  {}",
                    format!("#{}", routine.id).bright_black(),
                    routine.name,
                    format!("@{}", cron_schedule.upcoming(chrono::Local).next().unwrap())
                        .bright_black(),
                );
            }
        },

        _ => {},

        // Commands::Finish { id } => {
        //     state.finish_times.insert(
        //         id,
        //         Schedule::from_str(
        //             &schedule.routines.iter()
        //                 .filter(|r| r.id == id)
        //                 .next()
        //                 .unwrap_or(|_| panic!("Unable to find a task with id {}", id))
        //                 .period
        //         ).unwrap().upcoming(chrono::Local).next().unwrap(),
        //     );
        // },
    }
}
