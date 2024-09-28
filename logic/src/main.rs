use chrono::{DateTime, Local};
use colored::Colorize;
use std::{collections::HashMap, str::FromStr};
use heavy::{parse_cli, read_schedule, read_state, write_state, Commands, State, Cli};

struct App {
    schedule: heavy::Schedule,
    state: State,
}

impl App {
    fn new() -> Self {
        Self {
            schedule: read_schedule(),
            state: read_state(),
        }
    }

    fn show(&self) {
        println!("{}", "Upcoming:".bright_white());
        for routine in &self.schedule.routines {
            let Some(ref period) = routine.period else { continue };
            let time = cron::Schedule::from_str(&period)
                .unwrap()
                .after(self.state.finish_times.get(&routine.id).unwrap_or(&Local::now()))
                .next()
                .unwrap();

            println!(
                "{}  {}  {}",
                format!("#{}", routine.id).bright_black(),
                routine.name,
                format!("@{}", time).bright_black(),
            );
        }
    }

    fn done(&mut self, routine_id: &str) {
        let period = &self.schedule.routines.iter()
            .filter(|r| r.id == routine_id)
            .next()
            .unwrap_or_else(|| panic!("Unable to find a task with id {}", routine_id))
            .period.as_ref()
            .unwrap_or_else(|| panic!("No period specified for task with id {}", routine_id));

        self.state.finish_times.insert(
            routine_id.to_string(),
            cron::Schedule::from_str(period)
                .unwrap()
                .after(self.state.finish_times.get(routine_id).unwrap_or(&Local::now()))
                .next()
                .unwrap(),
        );
        write_state(&self.state);
    }
}

fn main() {
    let cli = parse_cli();
    let mut app = App::new();

    match cli.command {
        Commands::Show => { app.show(); },
        Commands::Done { ref id } => { app.done(id); },
    }
}
