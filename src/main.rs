mod calendar;
mod config;
mod flight;
mod flightstats;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "PilotTrack")]
#[command(version = "0.1.0")]
#[command(about = "App to track United pilots scheduled flights")]
struct Cli {
    /// Grab next flight from configured pilots calender schedule
    #[arg(short, long)]
    calendar: bool,
    /// Flight number of flight to track. Format: '1234'
    #[arg(short, long)]
    flight_number: String,
    /// Departure time. Format: 'MM/DD/YY' Defaults to current date and time
    #[arg(short, long)]
    departure_time: Option<String>,
}

fn main() -> Result<()> {
    // let filename = String::from("calendar.ics");

    // // Init config
    // let config = config::Config::from_env()?;

    // // Get .ics calendar from url
    // let calendar =
    //     calendar::get_calendar_from_url(&config.url, &config.username, &config.password)?;

    // calendar::save_calendar_to_file(&calendar, &filename)?;

    // let mut flight = calendar::get_next_flight(&calendar);
    // flight.live_update(flightstats::get_live_update(
    //     flight.scheduled_departure,
    //     &flight.flight_number,
    // )?);

    let cli: Cli = Cli::parse();

    // Parse or default requested departure time
    let departure_time = match cli.departure_time {
        Some(s) => chrono::NaiveDate::parse_from_str(&s, "%m/%d/%Y")?
            .and_hms_opt(0, 0, 0)
            .ok_or(anyhow::anyhow!(
                "Couldnt add default 'hms' to departure date"
            ))?,
        None => chrono::Local::now().naive_local(),
    };

    let mut flight = flight::Flight::default();
    flight.flight_number = Some(cli.flight_number);
    flight.scheduled_departure = Some(departure_time);

    flight.live_update(flightstats::get_live_update(
        &flight.scheduled_departure,
        &flight.flight_number,
    )?);
    println!("{flight:#?}");
    println!("Errors: {:#?}", flight.errors);

    Ok(())
}
