use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use reqwest::header::USER_AGENT;
use std::env;

#[allow(dead_code)]
struct FlightStatus {
    flight_number: Option<String>,
    dep_time: NaiveDateTime,
    arr_time: NaiveDateTime,
    dep_icao: String,
    arr_icao: String,
}

fn main() -> Result<()> {
    // Read env from file
    dotenvy::dotenv().ok();

    let username = env::var("UNUMBER").context("Username not found")?;
    let password = env::var("PASSWORD").context("Password not found")?;
    let url = env::var("URL").context("URL not found")?;

    dbg!(&username, &password, &url);

    // Get .ics
    let calendar = get_calendar(&url, &username, &password)?;

    // Parse calendar
    let calendar = calendar
        .parse::<Calendar>()
        .map_err(|e| anyhow::anyhow!("Failed to parse calendar: {e}"))?;

    let flight_number = if let Some(event) = get_next_event(&calendar) {
        get_flight_number(&event).ok()
    } else {
        None
    };

    Ok(())
}

fn get_flight_number(event: &Event) -> Result<String> {
    let flight_number = String::from("UAL")
        + String::from(event.get_summary().unwrap())
            .split(' ')
            .next()
            .unwrap();

    Ok(flight_number)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_get_response() {
        dotenvy::dotenv().ok();
        let username = env::var("UNUMBER").context("Username not found").unwrap();
        let password = env::var("PASSWORD").context("Password not found").unwrap();
        let url = env::var("URL").context("URL not found").unwrap();
        let cal = get_calendar(&url, &username, &password);
        assert!(cal.is_ok())
    }
}
