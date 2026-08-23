mod config;

use anyhow::Result;
use chrono::NaiveDateTime;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use reqwest::header::USER_AGENT;

#[allow(dead_code)]
struct FlightStatus {
    flight_number: Option<String>,
    dep_time: Option<NaiveDateTime>,
    arr_time: Option<NaiveDateTime>,
    dep_icao: Option<String>,
    arr_icao: Option<String>,
}

fn main() -> Result<()> {
    // Init config
    let config = config::Config::from_env()?;

    // Get .ics
    let calendar = get_calendar(&config.url, &config.username, &config.password)?;

    // Parse calendar
    let calendar = calendar
        .parse::<Calendar>()
        .map_err(|e| anyhow::anyhow!("Failed to parse calendar: {e}"))?;

    // Get flight number
    let flight_number = if let Some(event) = get_next_event(&calendar) {
        get_flight_number(&event)
    } else {
        None
    };

    println!("{flight_number:?}");

    Ok(())
}

fn get_flight_number(event: &Event) -> Option<String> {
    let flight_number = if let Some(summary) = event.get_summary() {
        if let Some(number) = summary.split(' ').next() {
            Some(String::from("UAL") + number)
        } else {
            return None;
        }
    } else {
        return None;
    };

    flight_number
}

fn get_next_event(cal: &Calendar) -> Option<&Event> {
    for event in cal.events() {
        if !is_complete(event) {
            return Some(event);
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
