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

    println!(
        "Sending request...\nU: {}\nP: {}\nURL: {}",
        username, password, url
    );

    let result = client
        .get(&url)
        .basic_auth(username, Some(password))
        .header(USER_AGENT, "curl/8.5.0")
        .send()?
        .text()?;

    // Parse calendar
    let parsed_calendar: Calendar = result.parse().unwrap();

    let current_time = chrono::Local::now().naive_local();
    for event in parsed_calendar.events() {
        match event.get_start() {
            Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone { date_time, tzid })) => {
                if date_time > current_time {
                    println!("{date_time}")
                }
            }
            _ => {}
        }
    }
    Ok(())
}
