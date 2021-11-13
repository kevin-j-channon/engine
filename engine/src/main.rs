use clap::{Arg, App};
use std::println;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::{Utc, Duration, DateTime};
use std::mem;

#[derive(Debug, Deserialize, Serialize)]
struct Position {
    latitude: f32,
    longitude: f32,
    elevation: f32
}

#[derive(Debug, Deserialize, Serialize)]
struct Orientation {
    direction: f32,
    slope: f32
}

#[derive(Debug, Deserialize, Serialize)]
struct SolarPanelArray {
    size: u32,
    position: Position,
    orientation: Orientation,
    nominal_power_w: f32
}

#[derive(Debug, Deserialize, Serialize)]
enum Supply {
    Solar { panels: std::vec::Vec<SolarPanelArray> },
    Wind {},
    Grid {}
}

#[derive(Debug, Deserialize, Serialize)]
struct Load {

}

#[derive(Debug, Deserialize, Serialize)]
struct Storage {

}

#[derive(Debug, Deserialize, Serialize)]
struct Configuration {
    run_length_days: u64,
    supplies: std::vec::Vec<Supply>,
    loads: std::vec::Vec<Load>,
    stores: std::vec::Vec<Storage>
}

/// Evaluate the state of the simulation at the specified time point.
fn evaluate_at_time_point(time_point: DateTime<Utc>, cfg: &Configuration) {
    println!("Evaluating at {:?}", time_point);
}

struct DateTimeIterator(DateTime<Utc>, DateTime<Utc>, Duration);

impl Iterator for DateTimeIterator {

    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next_item = self.0 + self.2;
            return Some(mem::replace(&mut self.0, next_item));
        } else {
            return None;
        }
    }
}

/// Evaluate everything.
fn evaluate(cfg: Configuration) -> Result<(), Box<dyn std::error::Error>> {

    println!("Run length = {} days", cfg.run_length_days);

    let start = Utc::now();
    let step = Duration::minutes(2);
    let end = start + Duration::days(cfg.run_length_days as i64);

    let total_steps = (end - start).num_minutes() / step.num_minutes();

    let mut time_point = start;
    for t in DateTimeIterator(start, end, step) {
        evaluate_at_time_point(t, &cfg);
        
        time_point = time_point + step;
    }

    return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Run calculations on various renewables scenarios for the house")
        .version("0.0.1")
        .author("Kevin Channon")
        .about("Fill me in later!")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Path to the config file for the app"))
        .get_matches();

    let config_file_path_str = matches.value_of("config").unwrap_or("default.json");
    println!("Using config from {}", config_file_path_str);

    let cfg: Configuration = serde_json::from_str(&fs::read_to_string(config_file_path_str)?)?;


    return evaluate(cfg);
}
