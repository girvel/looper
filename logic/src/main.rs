use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::{cmp::{max, Reverse}, collections::HashMap, str::FromStr};
use heavy::{parse_cli, read_schedule, read_state, write_state, Command, ConfigType, Routine, State};

/* TODO:
 *
 * x Install as executable
 * - Publish
 * x Redo schedule as a hashmap
 * x Display message on done
 * - --verbose flag to display more than 5 upcoming
 * x handle unwraps
 * x error displaying
 * - marking some tasks as immediate & valuable affecting colors & sorting
 * x quick schedule/state editing
 * x grouping tasks by periods in the schedule config
 * - check schedule ID collisions
 * - multiple arguments for `looper done`
 * x `looper` instead of `looper show`
 * - README
 * - Help message
 * - 1.0!
 * - dotfiles
 * - resolve TODOs
 * x bug: lp done for already finished tasks does not work
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
    fn new() -> Result<Self, String> {
        Ok(Self {
            schedule: read_schedule()?,
            state: read_state()?,
        })
    }

    fn get_next_date(&self, id: &str) -> Option<DateTime<Local>> {
        Some(cron::Schedule::from_str(&self.schedule.get(id)?.period)
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

    fn show(&self) -> Result<(), String> {
        let mut schedule_table: Vec<_> = self.schedule.iter()
            .filter_map(|(id, routine)| {
                let time = self.get_next_date(id)?;
                Some((id, &routine.name, time))
            })
            .collect();

        schedule_table.sort_by_key(|&(_, _, time)| Reverse(time));
        let tasks_to_do = {
            let mut result = vec![];
            loop {
                if schedule_table.last().map_or(true, |&(_, _, t)| t > Local::now()) { break; }
                let (id, name, _) = schedule_table.pop().unwrap();
                result.push((id, name));
            }
            result
        };

        println!(
            "\n{}",
            header(&format!(
                "[{}] Today is {}:",
                tasks_to_do.len(),
                Local::now().format(DATE_FORMAT)
            )),
        );

        for (id, name) in tasks_to_do {
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

        Ok(())
    }

    fn done(&mut self, routine_id: &str) -> Result<(), String> {
        let routine = &self.schedule.get(routine_id)
            .ok_or_else(|| format!("Unable to find a task with id {}", routine_id))?;

        let new_finish_time = max(self.get_next_date(routine_id).unwrap(), Local::now());

        self.state.finish_times.insert(routine_id.to_string(), new_finish_time);
        write_state(&self.state)?;

        println!("\n{}", header(&routine.name));
        println!("Done {}", date(&new_finish_time));
        println!("Next {}", date(&self.get_next_date(routine_id).unwrap()));

        Ok(())
    }

    fn path(&self, config_type: &ConfigType) -> Result<(), String> {
        println!("\n{}", config_type.get_path()?);
        Ok(())
    }
}

fn main() {
    let cli = parse_cli();
    App::new()
        .and_then(|mut app| match cli.command {
            None => { app.show() },
            Some(Command::Done { ref id }) => { app.done(id) },
            Some(Command::Path { ref config_type }) => { app.path(config_type) },
        })
        .unwrap_or_else(|message| println!("{}: {}", "ERROR".red(), message));
}
