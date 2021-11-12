use clap::{Arg, App};
use std::println;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct Configuration {
    run_length_days: u64
}

/// Evaluate everything.
fn evaluate(cfg: Configuration) -> Result<(), Box<dyn std::error::Error>> {

    println!("Run length = {} days", cfg.run_length_days);

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
