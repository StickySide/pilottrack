use anyhow::Result;
use icalendar::{Calendar, CalendarComponent, Component};
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

    for event in parsed_calendar.events() {
        println!("Event: {}", event.get_summary().unwrap())
    }
    Ok(())
}
