mod calendar;
mod config;

use anyhow::Result;
use chrono::NaiveDateTime;
use icalendar::Component;

#[allow(dead_code)]
struct FlightStatus {
    flight_number: Option<String>,
    dep_time: Option<NaiveDateTime>,
    arr_time: Option<NaiveDateTime>,
    dep_icao: Option<String>,
    arr_icao: Option<String>,
}

fn main() -> Result<()> {
    let filename = String::from("calendar.ics");

    // Init config
    let config = config::Config::from_env()?;

    // Get .ics calendar from url
    let calendar = calendar::from_url(&config.url, &config.username, &config.password)?;

    // Save to file
    calendar::to_file(calendar, &filename)?;

    // Load from file again!
    let calendar = calendar::from_file(&filename)?;

    // Next lines just for testing purposes
    let next_event = calendar::get_next_event(&calendar).unwrap();

    println!("{}", next_event.get_summary().unwrap());
    println!("{}", calendar::get_flight_number(next_event).unwrap());

    Ok(())
}
