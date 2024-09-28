use chrono::{DateTime, Local};
use colored::Colorize;
use std::{cmp::Reverse, str::FromStr};
use heavy::{parse_cli, read_schedule, read_state, write_state, Commands, State};

/* TODO:
 *
 * - Install as executable
 * - Publish
 * - Redo schedule as a hashmap
 * - Display some results on finish
 */

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
        let mut schedule_table: Vec<_> = self.schedule.routines.iter()
            .filter_map(|r| {
                let period = r.period.as_deref()?;
                let time = cron::Schedule::from_str(&period)
                    .unwrap()
                    .after(
                        self.state.finish_times
                            .get(&r.id)
                            .unwrap_or(&DateTime::<Local>::default())
                    )
                    .next()
                    .unwrap();

                Some((&r.id, &r.name, time))
            })
            .collect();

        schedule_table.sort_by_key(|&(_, _, time)| Reverse(time));

        println!("\n{}", format!("Today is {}:", Local::now().format("%d-%b-%Y")).bright_white());
        loop {
            if schedule_table.last().map_or(true, |&(_, _, t)| t > Local::now()) { break; }
            let (id, name, _) = schedule_table.pop().unwrap();

            println!(
                "{}  {}",
                format!("#{}", id).bright_black(),
                name,
                // format!("@{}", time.format("%d-%b-%Y")).bright_black(),
            );
        }

        println!("\n{}", "Upcoming:".bright_white());

        for (id, name, time) in schedule_table.iter().take(5) {
            println!(
                "{}  {}  {}",
                format!("#{}", id).bright_black(),
                name,
                format!("@{}", time.format("%d-%b-%Y")).bright_black(),
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
            if let Some(finish_time) = self.state.finish_times.get(routine_id) {
                cron::Schedule::from_str(period)
                    .unwrap()
                    .after(finish_time)
                    .next()
                    .unwrap()
            } else {
                Local::now()
            }
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
