use chrono::{DateTime, Local};
use std::{cmp::{max, Reverse}, path::PathBuf, str::FromStr};
use heavy::{
    cli::{parse, Command}, 
    config::{read_schedule, read_state, write_state, ConfigType, State, Schedule}
};


const UPCOMING_N: usize = 10;

// TODO group with module?
mod show {
    use chrono::{DateTime, Local};
    use colored::{ColoredString, Colorize};

    pub const DATE_FORMAT: &str = "%d-%b-%Y";
    pub const DATETIME_FORMAT: &str = "%d-%b-%Y %H:%M";

    pub fn header(text: &str) -> ColoredString {
        text.bright_white().bold()
    }

    pub fn date(date: &DateTime<Local>, show_time: bool) -> ColoredString {
        let format = if show_time { DATETIME_FORMAT } else { DATE_FORMAT };
        format!("@{}", date.format(format)).bright_black()
    }

    pub fn active_id(id: &str) -> ColoredString {
        format!("#{}", id).green()
    }

    pub fn inactive_id(id: &str) -> ColoredString {
        format!("#{}", id).bright_black()
    }

    pub fn error(message: &str) -> ColoredString {
        format!("{}: {}", "ERROR".red(), message).into()
    }
}

fn get_next_date(schedule: &Schedule, state: &State, id: &str)
    -> Result<DateTime<Local>, String>
{
    let cron_string = &schedule.get(id).ok_or(format!("Missing id {}", id))?.period;

    Ok(cron::Schedule::from_str(cron_string)
        .map_err(|err| err.to_string())?
        .after(
            state
                .get(id)
                .unwrap_or(&DateTime::<Local>::default())
        )
        .next()
        .unwrap()
    )
}

fn show(config_folder: Option<&PathBuf>, verbose: bool) -> Result<(), String> {
    let schedule = read_schedule(config_folder)?;
    let state = read_state(config_folder)?;

    let mut schedule_table = Vec::new();
    for (id, routine) in &schedule {
        let time = get_next_date(&schedule, &state, id)?;
        schedule_table.push((id, &routine.name, time));
    }
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
        show::header(&format!(
            "[{}] {}",
            tasks_to_do.len(),
            Local::now().format(show::DATE_FORMAT)
        )),
    );

    if tasks_to_do.is_empty() {
        println!("-- all done --");
    } else {
        for (id, name) in tasks_to_do {
            println!(
                "{}  {}",
                show::active_id(id),
                name,
            );
        }
    }

    if schedule_table.is_empty() { return Ok(()); }

    println!("\n{}", show::header("Upcoming:"));

    for (id, name, time)
    in schedule_table
        .iter()
        .rev()
        .take(if verbose { schedule_table.len() } else { UPCOMING_N })
    {
        println!(
            "{}  {}  {}",
            show::inactive_id(id),
            name,
            show::date(time, Local::now().date_naive() == time.date_naive()),
        );
    }

    if !verbose {
        if let Some(remaining_n) = schedule_table.len().checked_sub(UPCOMING_N) {
            println!("...{} more", remaining_n);
        }
    }

    Ok(())
}

fn done(config_folder: Option<&PathBuf>, routine_ids: &Vec<String>) -> Result<(), String> {
    let schedule = read_schedule(config_folder)?;
    let mut state = read_state(config_folder)?;

    for id in routine_ids {
        let routine = &schedule.get(id)
            .ok_or_else(|| format!("Unable to find a task with id {}", id))?;

        let new_finish_time = max(get_next_date(&schedule, &state, id).unwrap(), Local::now());

        state.insert(id.to_string(), new_finish_time);
        write_state(config_folder, &state)?;

        let new_next_time = get_next_date(&schedule, &state, id).unwrap();
        let show_time = new_next_time.date_naive() == new_finish_time.date_naive();

        println!("\n{}", show::header(&routine.name));
        println!("Done {}", show::date(&new_finish_time, show_time));
        println!("Next {}", show::date(&new_next_time, show_time));
    }

    Ok(())
}

fn path(config_folder: Option<&PathBuf>, config_type: &ConfigType) -> Result<(), String> {
    println!("\n{}", config_type.get_path(config_folder)?);
    Ok(())
}

fn main() {
    let cli = parse();
    match cli.command {
        Command::Show { verbose } => { show(cli.config_folder.as_ref(), verbose) },
        Command::Done { ref ids } => { done(cli.config_folder.as_ref(), ids) },
        Command::Path { ref config_type } => { path(cli.config_folder.as_ref(), config_type) },
    }
    .unwrap_or_else(|message| println!("{}", show::error(&message)));
}
