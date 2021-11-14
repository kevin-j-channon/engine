use std::println;
use serde::{Serialize, Deserialize};
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

impl SolarPanelArray {
    
    #[allow(dead_code)]
    pub fn new(size: u32, location: Location, orientation: Orientation, nominal_power_w: f32) -> SolarPanelArray{
        return SolarPanelArray{
            size,
            location,
            orientation,
            nominal_power_w};
    }

    /// The incident intensity factor is a factor that accounts for the location & orientation of the panel array
    /// (on the globe) and the time of day & date. It is a value between zero and one.
    pub(crate) fn incident_intensity_factor(&self, _time: &DateTime<Utc>) -> f32 {
        return 1.0
    }

    /// Get the output of the array.
    pub fn output(&self, time: &DateTime<Utc>, _weather: &Weather) -> f32 {
        return self.nominal_power_w * self.incident_intensity_factor(time);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn solar_panel_array_incident_intensity_factor_is_correct() {
        let array = SolarPanelArray::new(8, Location{ latitude: 0.0, longitude: 0.0, elevation: 0.0}, Orientation{direction: 0.0, slope: 0.0}, 300.0);
        let time = DateTime::parse_from_str("2021-11-14T00:00:00+0000", "%Y-%m-%dT%H:%M:%S%z").unwrap().with_timezone(&Utc);

        assert_eq!(1.0, array.incident_intensity_factor(&time));
    }

    #[test]
    fn solar_panel_array_output_is_correct() {
        let array = SolarPanelArray::new(8, Location{ latitude: 0.0, longitude: 0.0, elevation: 0.0}, Orientation{direction: 0.0, slope: 0.0}, 300.0);
        let time = DateTime::parse_from_str("2021-11-14T00:00:00+0000", "%Y-%m-%dT%H:%M:%S%z").unwrap().with_timezone(&Utc);

        let weather = Weather{};

        assert_eq!(300.0, array.output(&time, &weather));
    }
}

struct Weather {}

impl Weather {
    #[allow(dead_code)]
    fn temperature_c(_time: &DateTime<Utc>, _location: &Location) ->f32 {
        return 20.0;
    }

    #[allow(dead_code)]
    fn cloud_cover(_time: &DateTime<Utc>, _location: &Location) -> f32 {
        return 0.0;
    }

    #[allow(dead_code)]
    fn wind_speed_ms(_time: &DateTime<Utc>, _location: &Location) -> f32 {
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
pub struct Configuration {
    run_length_days: u64,
    supplies: std::vec::Vec<Supply>,
    loads: std::vec::Vec<Load>,
    stores: std::vec::Vec<Storage>
}

fn calculate_supply(time_point: &DateTime<Utc>, weather: &Weather, supplies: &std::vec::Vec<Supply>) -> (){
    let mut _supplied_amount: f32 = 0.0;
    // let mut cost: f32 = 0.0;
    for supply in supplies {
        match supply {
            Supply::Solar{ panels } => { 
                for array in panels {
                    _supplied_amount += array.output(time_point, weather);
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

        let _generation = calculate_supply(&time_point, weather, supplies);
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
pub fn evaluate(cfg: Configuration) -> Result<(), Box<dyn std::error::Error>> {

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