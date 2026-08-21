use anyhow::{Context, Result};
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use reqwest::header::USER_AGENT;
use std::env;

fn main() -> Result<()> {
    // Read env from file
    dotenvy::dotenv().ok();

    let username = env::var("USERNAME").context("Username not found")?;
    let password = env::var("PASSWORD").context("Password not found")?;
    let url = env::var("URL")?;

    // Get .ics
    let calendar = get_calendar(&url, &username, &password)?;

    // Parse calendar
    let calendar: Calendar = calendar.parse().unwrap();

    // Prints summary for next event
    println!(
        "{}",
        get_next_event(&calendar).unwrap().get_summary().unwrap()
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
    if let Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone {
        date_time: dt,
        tzid: _,
    })) = event.get_end()
        && dt.and_utc() < current_time.and_utc()
    {
        return true;
    }
    false
}

fn get_calendar(url: &str, username: &str, password: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder().build()?;

    let response = client
        .get(url)
        .basic_auth(username, Some(password))
        .header(USER_AGENT, "curl/8.5.0")
        .send()?
        .text()?;

    Ok(response)
}
