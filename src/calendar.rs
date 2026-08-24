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
        .map_err(|e| anyhow::anyhow!("Failed to parse calendar from url: {e}"))?;

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

pub fn to_file(calendar: Calendar, filename: &String) -> std::io::Result<()> {
    std::fs::write(filename, calendar.to_string())
}

pub fn from_file(filename: &String) -> Result<Calendar> {
    let cal_string = std::fs::read_to_string(filename)?;
    let calendar: Calendar = cal_string
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse calendar from file: {e}"))?;
    Ok(calendar)
}

#[cfg(test)]
mod tests {}
