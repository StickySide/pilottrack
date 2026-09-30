use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(name = "PilotTrack")]
#[command(version = "0.1.0")]
#[command(about = "App to track United pilots scheduled flights")]
pub struct Cli {
    #[command(flatten)]
    source: Source,
    /// Departure time. Format: 'MM/DD/YY' Defaults to current date and time
    #[arg(short, long)]
    departure_time: Option<String>,
    /// Filename to use for saved calendar.
    #[arg(long, default_value_t = String::from("calendar.ics"))]
    filename: String,
    /// Ratatui test run
    #[arg(short, long)]
    ratatui: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Source {
    /// Grab next flight from configured pilots calender schedule
    #[arg(short, long)]
    pub calendar: bool,

    /// Flight number of flight to track. Format: '1234'
    #[arg(short, long)]
    pub flight_number: Option<String>,
}
