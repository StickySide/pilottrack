use anyhow::Result;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use reqwest::header::USER_AGENT;
use std::env;

fn main() -> Result<()> {
    // Read env from file
    dotenvy::dotenv().ok();

    let username = env::var("USERNAME")?;
    let password = env::var("PASSWORD")?;
    let url = env::var("URL")?;

    // Get .ics
    let client = reqwest::blocking::Client::builder().build()?;

    println!("Getting calendar...");

    let result = client
        .get(&url)
        .basic_auth(username, Some(password))
        .header(USER_AGENT, "curl/8.5.0")
        .send()?
        .text()?;

    // Parse calendar
    let parsed_calendar: Calendar = result.parse().unwrap();

    // Prints summary for next event
    println!(
        "{}",
        get_next_event(&parsed_calendar)
            .unwrap()
            .get_summary()
            .unwrap()
    );

    Ok(())
}

fn get_next_event(cal: &Calendar) -> Option<Event> {
    for event in cal.events() {
        if !is_complete(event) {
            return Some(event.clone());
        }
    }
    None
}

fn is_complete(event: &Event) -> bool {
    let current_time = chrono::Local::now().naive_local();
    if let Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone { date_time, tzid })) =
        event.get_end()
        && date_time.and_utc() < current_time.and_utc()
    {
        return true;
    }
    false
}
