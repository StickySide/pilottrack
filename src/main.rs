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

    // Get .ics calendar
    let calendar = calendar::from_url(&config.url, &config.username, &config.password)?;

    calendar::to_file(calendar, &filename)?;

    let calendar = calendar::from_file(&filename)?;

    for event in calendar.events() {
        println!("{}", event.get_summary().unwrap());

        match calendar::get_flight_number(event) {
            Some(n) => println!("{n}"),
            None => println!("Could not get flight number"),
        }
    }
    // Get flight number
    // let flight_number = if let Some(event) = calendar::get_next_event(&calendar) {
    //     calendar::get_flight_number(&event)
    // } else {
    //     None
    // };

    // dbg!(flight_number);

    Ok(())
}
