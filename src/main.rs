mod calendar;
mod config;
mod flight;
mod flightstats;

use anyhow::Result;

fn main() -> Result<()> {
    let filename = String::from("calendar.ics");

    // Init config
    let config = config::Config::from_env()?;

    // Get .ics calendar from url
    let calendar =
        calendar::get_calendar_from_url(&config.url, &config.username, &config.password)?;

    calendar::save_calendar_to_file(&calendar, &filename)?;

    let mut flight = calendar::get_next_flight(&calendar);
    flight.live_update(flightstats::get_live_update(
        flight.scheduled_departure,
        flight.callsign(),
    )?);

    println!("{flight}");

    Ok(())
}
