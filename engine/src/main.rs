use clap::{Arg, App};
use std::println;

fn main() {
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

    let config_file = matches.value_of("config").unwrap_or("default.json");
    println!("Using config from {}", config_file);
}
