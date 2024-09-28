use cron::Schedule;
use std::str::FromStr;

fn main() {
    let _cli = heavy::parse_cli();
    let schedule = heavy::read_config();

    println!("Upcoming:");
    for routine in schedule.routines {
        let cron_schedule = Schedule::from_str(&routine.period).unwrap();
        // TODO coloring
        println!(
            "#{}  {}  @{}",
            routine.id,
            routine.name,
            cron_schedule.upcoming(chrono::Local).next().unwrap(),
        );
    }
}
