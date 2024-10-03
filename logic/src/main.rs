use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::{cmp::Reverse, collections::HashMap, str::FromStr};
use heavy::{parse_cli, read_schedule, read_state, write_state, Commands, Routine, State};

/* TODO:
 *
 * x Install as executable
 * - Publish
 * x Redo schedule as a hashmap
 * x Display message on done
 * - --verbose flag to display more than 5 upcoming
 * - handle unwraps
 * - error handling
 * - grouping tasks by periods in the schedule config
 * - README
 * - Help message
 * - 1.0!
 * - dotfiles
 */

struct App {
    schedule: HashMap<String, Routine>,
    state: State,
}

const DATE_FORMAT: &str = "%d-%b-%Y";

fn header(text: &str) -> ColoredString {
    text.bright_white().bold()
}

fn date(date: &DateTime<Local>) -> ColoredString {
    format!("@{}", date.format(DATE_FORMAT)).bright_black()
}

impl App {
    fn new() -> Self {
        Self {
            schedule: read_schedule(),
            state: read_state(),
        }
    }

    fn get_next_date(&self, id: &str) -> Option<DateTime<Local>> {
        let period = self.schedule.get(id)?.period.as_deref()?;
        Some(cron::Schedule::from_str(period)
            .unwrap()
            .after(
                self.state.finish_times
                    .get(id)
                    .unwrap_or(&DateTime::<Local>::default())
            )
            .next()
            .unwrap()
        )
    }

    fn show(&self) {
        let mut schedule_table: Vec<_> = self.schedule.iter()
            .filter_map(|(id, routine)| {
                let time = self.get_next_date(id)?;
                Some((id, &routine.name, time))
            })
            .collect();

        schedule_table.sort_by_key(|&(_, _, time)| Reverse(time));

        println!(
            "\n{}",
            header(&format!("Today is {}:", Local::now().format(DATE_FORMAT))),
        );

        loop {
            if schedule_table.last().map_or(true, |&(_, _, t)| t > Local::now()) { break; }
            let (id, name, _) = schedule_table.pop().unwrap();

            println!(
                "{}  {}",
                format!("#{}", id).bright_black(),
                name,
            );
        }

        println!("\n{}", header("Upcoming:"));

        for (id, name, time) in schedule_table.iter().rev().take(5) {
            println!(
                "{}  {}  {}",
                format!("#{}", id).bright_black(),
                name,
                date(time),
            );
        }

        if let Some(remaining_n) = schedule_table.len().checked_sub(5usize) {
            println!("...{} more", remaining_n);
        }
    }

    fn done(&mut self, routine_id: &str) {
        let routine = &self.schedule.get(routine_id)
            .unwrap_or_else(|| panic!("Unable to find a task with id {}", routine_id));

        let period = routine.period.as_ref()
            .unwrap_or_else(|| panic!("No period specified for task with id {}", routine_id));

        let new_finish_time = if let Some(finish_time) = self.state.finish_times.get(routine_id) {
            cron::Schedule::from_str(period)
                .unwrap()
                .after(finish_time)
                .next()
                .unwrap()
        } else {
            Local::now()
        };

        self.state.finish_times.insert(routine_id.to_string(), new_finish_time.clone());
        write_state(&self.state);

        println!("\n{}", header(&routine.name));
        println!("Done {}", date(&new_finish_time));
        println!("Next {}", date(&self.get_next_date(routine_id).unwrap()));
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
