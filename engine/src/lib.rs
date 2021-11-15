use std::println;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration, DateTime, NaiveDate};
use std::mem;
extern crate nalgebra as na;
use na::{Vector3, Rotation3};

const EARTH_AXIS_TILT_RAD: f32 = 2.0 * std::f32::consts::PI * 23.46 / 360.0;
const Ω_DAY: f32 = 2.0 * std::f32::consts::PI / ((4.0 + 56.0 * 60.0 + 3600.0 * 23.0) / 60.0);
const Ω_YEAR: f32 = Ω_DAY / 365.256;

pub(crate) const X: usize = 0;
pub(crate) const Y: usize = 1;
pub(crate) const Z: usize = 2;

// let REFERENCE_TIME_POINT: DateTime<Utc> = ;

pub(crate) trait SimulationTimeIndex {
    fn from_datetime(time: &DateTime<Utc>) -> f32;
}

impl SimulationTimeIndex for DateTime<Utc> {
    fn from_datetime(time: &DateTime<Utc>) -> f32 {
        return (*time - DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 6, 22).and_hms(12, 0, 0), Utc).with_timezone(&Utc)).num_minutes() as f32;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Location {
    latitude: f32,
    longitude: f32,
    elevation: f32
}

impl Location {
    pub(crate) fn new(lat: f32, lon: f32, ele: f32) -> Location {
        return Location{
            latitude: lat.to_radians(),
            longitude: lon.to_radians(),
            elevation: ele.to_radians()
        };
    }

    pub(crate) fn normal(&self, time_idx: &f32) -> na::Vector3<f32> {
        return na::vector![
            self.latitude.cos() * (Ω_DAY * time_idx).cos(),
            self.latitude.cos() * (Ω_DAY * time_idx).sin(),
            self.latitude.sin()];
    }
}

pub(crate) trait Normal {
    /// Create a Normal object from a Location.
    fn from_location(location: Location) -> Self;

    /// Rotate a normal vector based on the time of day.
    fn apply_daytime_rotation(self, time_idx: &f32) -> Self;

    /// Rotate a normal vector based on the tilt of the earths axis.
    fn apply_planetary_axis_tilt(self) -> Self;

    /// Rotate a normal according to the orientation of a surface normal.
    fn apply_surface_normal_rotation(self, orientation: Orientation) -> Self;
}

impl Normal for na::Vector3<f32> {
    fn from_location(location: Location) -> Self {
        return na::vector![
            location.latitude.cos() * location.longitude.cos(),
            location.latitude.cos() * location.longitude.sin(),
            location.latitude.sin()
            ];
    }

    fn apply_daytime_rotation(self, time_idx: &f32) -> Self {
        return self;
    }

    fn apply_planetary_axis_tilt(self) -> Self {
        return self;
    }

    fn apply_surface_normal_rotation(self, orientation: Orientation) -> Self {
        return self;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Orientation {
    direction: f32,
    slope: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SolarPanelArray {
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
    pub(crate) fn incident_intensity_factor(&self, time: &DateTime<Utc>) -> f32 {
        let θ: &f32 = &self.location.latitude;   // Lattitude of locaiton.

        let t = (*time - DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 6, 22).and_hms(0, 0, 0), Utc).with_timezone(&Utc)).num_minutes() as f32;
         println!("time = {:?}", time);
         println!("t = {}", t);
         println!("θ.cos() = {}", θ.cos());
         println!("θ.sin() = {}", θ.sin());
         println!("φ.cos() = {}", (-EARTH_AXIS_TILT_RAD).cos());
         println!("φ.sin() = {}", (-EARTH_AXIS_TILT_RAD).sin());

        let out = (Ω_YEAR * t).cos() * (-EARTH_AXIS_TILT_RAD).cos() * θ.sin() * (Ω_DAY * t).cos()
                - (Ω_YEAR * t).cos() * (-EARTH_AXIS_TILT_RAD).sin() * θ.cos()
                + (Ω_YEAR * t).sin() * θ.sin() * (Ω_DAY * t).sin();

        return if out > 0.0 { out } else { 0.0 };
    }

    /// Get the output of the array.
    pub fn output(&self, time: &DateTime<Utc>, _weather: &Weather) -> f32 {
        return self.nominal_power_w * self.incident_intensity_factor(time);
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
#[serde(tag = "type")]
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn location_normal_is_correct_north_pole() {
        let location = Location::new( 90.0, 0.0, 0.0);

        let normal = na::Vector3::<f32>::from_location(location);

        assert!((0.0 - normal[X]).abs() < 0.0001);
        assert!((0.0 - normal[Y]).abs() < 0.0001);
        assert_eq!(1.0, normal[Z]);
    }

    #[test]
    fn solar_panel_array_incident_intensity_factor_is_correct() {
        let angle = std::f32::consts::FRAC_PI_2 - EARTH_AXIS_TILT_RAD;
        let array = SolarPanelArray::new(8, Location{ latitude: angle, longitude: 0.0, elevation: 0.0}, Orientation{direction: 0.0, slope: 0.0}, 300.0);
        let time = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 6, 22).and_hms(0, 0, 0), Utc).with_timezone(&Utc);

        assert_eq!(1.0, array.incident_intensity_factor(&time));
    }

    #[test]
    fn solar_panel_array_incident_intensity_factor_is_correct_2() {
        let angle = -EARTH_AXIS_TILT_RAD;
        let array = SolarPanelArray::new(8, Location{ latitude: angle, longitude: 0.0, elevation: 0.0}, Orientation{direction: 0.0, slope: 0.0}, 300.0);
        let time = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 6, 22).and_hms(0, 0, 0), Utc).with_timezone(&Utc);

        assert_eq!(0.0, array.incident_intensity_factor(&time));
    }

    #[test]
    fn solar_panel_array_output_is_correct() {
        let angle = std::f32::consts::FRAC_PI_2 - EARTH_AXIS_TILT_RAD;
        let array = SolarPanelArray::new(8, Location{ latitude: angle, longitude: 0.0, elevation: 0.0}, Orientation{direction: 0.0, slope: 0.0}, 300.0);
        let time = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 6, 22).and_hms(0, 0, 0), Utc).with_timezone(&Utc);

        let weather = Weather{};

        assert_eq!(300.0, array.output(&time, &weather));
    }
}