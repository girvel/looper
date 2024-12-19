use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::{cmp::{max, Reverse}, str::FromStr};
use heavy::{
    cli::{parse, Command}, 
    config::{read_schedule, read_state, write_state, ConfigType, State, Schedule}
};


const DATE_FORMAT: &str = "%d-%b-%Y";
const UPCOMING_N: usize = 10;

fn header(text: &str) -> ColoredString {
    text.bright_white().bold()
}

fn date(date: &DateTime<Local>) -> ColoredString {
    format!("@{}", date.format(DATE_FORMAT)).bright_black()
}

fn get_next_date(schedule: &Schedule, state: &State, id: &str)
    -> Result<DateTime<Local>, String>
{
    let cron_string = &schedule.get(id).ok_or(format!("Missing id {}", id))?.period;

    Ok(cron::Schedule::from_str(cron_string)
        .unwrap()
        .after(
            state
                .get(id)
                .unwrap_or(&DateTime::<Local>::default())
        )
        .next()
        .unwrap()
    )
}

fn show(verbose: bool) -> Result<(), String> {
    let schedule = read_schedule()?;
    let state = read_state()?;

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
        header(&format!(
            "[{}] {}",
            tasks_to_do.len(),
            Local::now().format(DATE_FORMAT)
        )),
    );

    if tasks_to_do.is_empty() {
        println!("-- all done --");
    } else {
        for (id, name) in tasks_to_do {
            println!(
                "{}  {}",
                format!("#{}", id).green(),
                name,
            );
        }
    }

    if schedule_table.is_empty() { return Ok(()); }

    println!("\n{}", header("Upcoming:"));

    for (id, name, time)
    in schedule_table
        .iter()
        .rev()
        .take(if verbose { schedule_table.len() } else { UPCOMING_N })
    {
        println!(
            "{}  {}  {}",
            format!("#{}", id).bright_black(),
            name,
            date(time),
        );
    }

    if !verbose {
        if let Some(remaining_n) = schedule_table.len().checked_sub(UPCOMING_N) {
            println!("...{} more", remaining_n);
        }
    }

    Ok(())
}

fn done(routine_ids: &Vec<String>) -> Result<(), String> {
    let schedule = read_schedule()?;
    let mut state = read_state()?;

    for id in routine_ids {
        let routine = &schedule.get(id)
            .ok_or_else(|| format!("Unable to find a task with id {}", id))?;

        let new_finish_time = max(get_next_date(&schedule, &state, id).unwrap(), Local::now());

        state.insert(id.to_string(), new_finish_time);
        write_state(&state)?;

        println!("\n{}", header(&routine.name));
        println!("Done {}", date(&new_finish_time));
        println!("Next {}", date(&get_next_date(&schedule, &state, id).unwrap()));
    }

    Ok(())
}

fn path(config_type: &ConfigType) -> Result<(), String> {
    println!("\n{}", config_type.get_path()?);
    Ok(())
}

fn main() {
    let cli = parse();
    match cli.command {
        Command::Show { verbose } => { show(verbose) },
        Command::Done { ref ids } => { done(ids) },
        Command::Path { ref config_type } => { path(config_type) },
    }
    .unwrap_or_else(|message| println!("{}: {}", "ERROR".red(), message));
}
