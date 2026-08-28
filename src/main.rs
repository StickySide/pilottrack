mod calendar;
mod config;
mod flightstats;

use anyhow::Result;
use chrono::NaiveDateTime;
use icalendar::{Component, DatePerhapsTime};

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

    dbg!(&calendar);

    // Next lines just for testing purposes
    let next_event =
        calendar::get_next_event(&calendar).ok_or(anyhow::anyhow!("Next event was empty"))?;

    dbg!(&next_event);

    // Get flight stats
    let flight_number = calendar::get_flight_number(next_event).unwrap();

    dbg!(&flight_number);

    let start_time = calendar::get_start_ndt(next_event)?;

    let flight_stat_data = flightstats::from_url(start_time, flight_number)?;

    dbg!(&flight_stat_data);

    let flight_stats = flightstats::parse(flight_stat_data)?;

    // Display flight stats

    println!("{flight_stats}");
    Ok(())
}
