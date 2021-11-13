use clap::{Arg, App};
use std::println;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::{Utc, Duration, DateTime};
use std::mem;

#[derive(Debug, Deserialize, Serialize)]
struct Location {
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
    location: Location,
    orientation: Orientation,
    nominal_power_w: f32
}

struct Weather {}

impl SolarPanelArray {
    pub fn new(size: u32, location: Location, orientation: Orientation, nominal_power_w: f32) -> SolarPanelArray{
        return SolarPanelArray{
            size,
            location,
            orientation,
            nominal_power_w};
    }

    /// Get the output of the array.
    pub fn output(&self, time: &DateTime<Utc>, weather: &Weather) -> f64 {
        return 0.0;
    }
}

impl Weather {
    fn temperature_c(time: &DateTime<Utc>, location: &Location) ->f32 {
        return 20.0;
    }

    fn cloud_cover(time: &DateTime<Utc>, location: &Location) -> f32 {
        return 0.0;
    }

    fn wind_speed_ms(time: &DateTime<Utc>, location: &Location) -> f32 {
        return 0.0;
    }
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

fn calculate_solar(time_point: &DateTime<Utc>, weather: &Weather, solar_arrays: &std::vec::Vec<SolarPanelArray>, supplied_amount: &mut f64, cost: &mut f32) {
    
}

fn calculate_supply(time_point: &DateTime<Utc>, weather: &Weather, supplies: &std::vec::Vec<Supply>) -> (){
    let mut supplied_amount: f64 = 0.0;
    let mut cost: f32 = 0.0;
    for supply in supplies {
        match supply {
            Supply::Solar{ panels } => { 
                for array in panels {
                    supplied_amount += array.output(time_point, weather);
                }
            }
            Supply::Wind{} => {}
            Supply::Grid{} => {}
        }
    }
}

/// Evaluate the state of the simulation at the specified time point.
fn evaluate_at_time_point(
    time_point: DateTime<Utc>,
    weather: &Weather,
    supplies: &std::vec::Vec<Supply>,
    _loads: &std::vec::Vec<Load>,
    _stores: &std::vec::Vec<Storage>) {

        let generation = calculate_supply(&time_point, weather, supplies);
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

    let weather = Weather{};

    let mut time_point = start;
    for t in DateTimeIterator(start, end, step) {
        evaluate_at_time_point(t, &weather, &cfg.supplies, &cfg.loads, &cfg.stores);
        
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
