mod calendar;
mod config;

use anyhow::Result;
use chrono::NaiveDateTime;

#[allow(dead_code)]
struct FlightStatus {
    flight_number: Option<String>,
    dep_time: Option<NaiveDateTime>,
    arr_time: Option<NaiveDateTime>,
    dep_icao: Option<String>,
    arr_icao: Option<String>,
}

fn main() -> Result<()> {
    // Init config
    let config = config::Config::from_env()?;

    // Get .ics calendar
    let calendar = calendar::from_url(&config.url, &config.username, &config.password)?;

    // Get flight number
    let flight_number = if let Some(event) = calendar::get_next_event(&calendar) {
        calendar::get_flight_number(&event)
    } else {
        None
    };

    dbg!(flight_number);

    Ok(())
}
