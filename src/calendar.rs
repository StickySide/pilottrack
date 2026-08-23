use anyhow::Result;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};

pub fn from_url(url: &str, username: &str, password: &str) -> Result<icalendar::Calendar> {
    let client = reqwest::blocking::Client::builder().build()?;

    let response = client
        .get(url)
        .basic_auth(username, Some(password))
        .header(reqwest::header::USER_AGENT, "curl/8.5.0")
        .send()?
        .text()?;

    let calendar = response
        .parse::<icalendar::Calendar>()
        .map_err(|e| anyhow::anyhow!("Failed to parse calendar: {e}"))?;

    Ok(calendar)
}

pub fn get_next_event(cal: &Calendar) -> Option<&Event> {
    for event in cal.events() {
        if !is_complete(event) {
            return Some(event);
        }
    }
    None
}

pub fn get_flight_number(event: &Event) -> Option<String> {
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
