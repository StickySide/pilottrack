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
        &flight.flight_number,
    )?);

    // let mut flight = flight::Flight::default();
    // flight.flight_number = Some("1389".to_string());
    // flight.scheduled_departure = Some(chrono::Local::now().naive_local());

    // flight.live_update(flightstats::get_live_update(
    //     flight.scheduled_departure,
    //     flight.flight_number.clone(),
    // )?);
    println!("{flight}");
    println!("{:#?}", flight.errors);

    Ok(())
}
