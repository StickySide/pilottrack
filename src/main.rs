mod calendar;
mod config;
mod flight;
mod flightstats;

use anyhow::Result;
use chrono::NaiveDateTime;

fn main() -> Result<()> {
    let filename = String::from("calendar.ics");

    // Init config
    let config = config::Config::from_env()?;

    // Get .ics calendar from url
    let calendar =
        calendar::get_calendar_from_url(&config.url, &config.username, &config.password)?;

    calendar::to_file(&calendar, &filename)?;

    let flight = calendar::get_next_flight(&calendar);

    println!("{flight}");

    Ok(())
}
