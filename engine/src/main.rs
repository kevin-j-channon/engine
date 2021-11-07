use clap::{Arg, App};
use std::println;
use std::fs;
use json;

/// Evaluate everything.
fn evaluate(config: json::JsonValue) -> Result<(), Box<dyn std::error::Error>> {

    

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

    let config = json::parse(&fs::read_to_string(config_file_path_str)?)?;

    return evaluate(config);
}
