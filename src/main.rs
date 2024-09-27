use std::{env, fs};

use toml::Table;

fn main() {
    let config_path = format!("{}/.config/looper/schedule.toml", env::var("HOME").unwrap());
    let config = fs::read_to_string(&config_path)
        .expect(format!("No configuration file at {}", &config_path).as_str());
    println!("{}", config);
    // let config = "foo = 'bar'".parse::<Table>().unwrap();
    // println!("foo is {}", config["foo"].as_str().unwrap());
}
