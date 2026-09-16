mod calendar;
mod config;
mod flight;
mod flightstats;

use anyhow::Result;
use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(name = "PilotTrack")]
#[command(version = "0.1.0")]
#[command(about = "App to track United pilots scheduled flights")]
struct Cli {
    #[command(flatten)]
    source: Source,
    /// Departure time. Format: 'MM/DD/YY' Defaults to current date and time
    #[arg(short, long)]
    departure_time: Option<String>,
    /// Filename to use for saved calendar.
    #[arg(long, default_value_t = String::from("calendar.ics"))]
    filename: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Source {
    /// Grab next flight from configured pilots calender schedule
    #[arg(short, long)]
    calendar: bool,

    /// Flight number of flight to track. Format: '1234'
    #[arg(short, long)]
    flight_number: Option<String>,
}

fn main() -> Result<()> {
    // // Get .ics calendar from url

    let cli: Cli = Cli::parse();

    // Get flight from calendar or one-shot
    let mut flight = {
        if cli.source.calendar == true {
            // Init config
            let config = config::Config::from_env()?;
            let filename = cli.filename;

            // Grab calendar from United
            let calendar =
                calendar::get_calendar_from_url(&config.url, &config.username, &config.password)?;

            // Save calendar
            calendar::save_calendar_to_file(&calendar, &filename)?;

            // Grab next flight from calendar
            let flight = calendar::get_next_flight(&calendar);

            flight
        } else {
            // Parse or default requested departure time
            let departure_time = match cli.departure_time {
                Some(s) => chrono::NaiveDate::parse_from_str(&s, "%m/%d/%Y")?
                    .and_hms_opt(0, 0, 0)
                    .ok_or(anyhow::anyhow!(
                        "Couldnt add default 'hms' to departure date:"
                    ))?,
                None => chrono::Local::now().naive_local(),
            };

            let mut flight = flight::Flight::default();
            // Todo: Handle this unwrap
            flight.flight_number = Some(cli.source.flight_number.unwrap());
            flight.scheduled_departure = Some(departure_time);
            flight
        }
    };

    // Update flight with live info

    let live_update =
        flightstats::get_live_update(&flight.scheduled_departure, &flight.flight_number)?;
    flight.live_update(live_update);
    println!("{flight:#?}");

    Ok(())
}
