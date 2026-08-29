use crate::flight::Flight;
use anyhow::Result;
use chrono::NaiveDateTime;
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};

pub fn get_next_flight(cal: &Calendar) -> Flight {
    let event = get_next_event(&cal).unwrap();
    let number = get_flight_number(&event);
    // todo: need timezone info
    let scheduled_departure = get_start_ndt(event);
    let scheduled_arrival = get_end_ndt(event);
    Flight {
        number,
        scheduled_departure,
        scheduled_arrival,
    }
}

pub fn get_calendar_from_url(
    url: &str,
    username: &str,
    password: &str,
) -> Result<icalendar::Calendar> {
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

pub fn from_file(filename: &String) -> Result<Calendar> {
    let cal_string = std::fs::read_to_string(filename)?;
    let calendar: Calendar = cal_string
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse icalendar from file: {e}"))?;
    Ok(calendar)
}

pub fn get_next_event(cal: &Calendar) -> Option<&Event> {
    cal.events()
        .filter(|event| !is_complete(event))
        .min_by_key(|event| get_start_ndt(event))
}

// todo: change return type to Result? Maybe?
pub fn get_flight_number(event: &Event) -> Option<String> {
    let summary = event.get_summary()?;
    let number = summary.split_whitespace().next()?;

    Some(String::from(number))
}

// todo: should this be a result?
pub fn get_start_ndt(event: &Event) -> Option<NaiveDateTime> {
    match event.get_start() {
        Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone {
            date_time: dt,
            tzid: _,
        })) => Some(dt),
        _ => None,
    }
}

pub fn get_end_ndt(event: &Event) -> Option<NaiveDateTime> {
    match event.get_end() {
        Some(DatePerhapsTime::DateTime(CalendarDateTime::WithTimezone {
            date_time: dt,
            tzid: _,
        })) => Some(dt),
        _ => None,
    }
}
// todo: change return type to result or option
fn is_complete(event: &Event) -> bool {
    let current_time = chrono::Utc::now();
    let end = match event.get_end() {
        Some(DatePerhapsTime::DateTime(dt)) => dt.try_into_utc().unwrap(),
        _ => return false,
    };

    current_time > end
}

pub fn to_file(calendar: &Calendar, filename: &String) -> std::io::Result<()> {
    std::fs::write(filename, calendar.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::{Duration, Utc};
    use icalendar::{Component, EventLike};

    #[test]
    fn event_not_complete() {
        let event = icalendar::Event::new()
            .summary("Test event")
            .starts(Utc::now() - Duration::hours(1))
            .ends(Utc::now() + Duration::hours(1))
            .done();

        assert!(!is_complete(&event));
    }

    #[test]
    fn event_complete() {
        let event = icalendar::Event::new()
            .summary("Test event")
            .starts(Utc::now() - Duration::hours(2))
            .ends(Utc::now() - Duration::hours(1))
            .done();

        assert!(is_complete(&event));
    }

    #[test]
    fn parse_flight_number() {
        let event = icalendar::Event::new()
            .summary("1234 EWR 08Sep 08:00 - SFO 08Sep 10:58")
            .done();

        let flight_number = get_flight_number(&event);
        assert_eq!(flight_number, Some(String::from("1234")))
    }

    #[test]
    fn get_ndt() {
        let calendar = from_file(&"tests/fixtures/calendar.ics".to_string()).unwrap();
        let next_event = get_next_event(&calendar).unwrap();
        let dt = get_start_ndt(next_event).unwrap();
        let test_dt = chrono::NaiveDate::from_ymd_opt(2026, 08, 23)
            .unwrap()
            .and_hms_opt(19, 35, 22)
            .unwrap();
        assert_eq!(dt, test_dt);
    }

    #[test]
    fn get_calendar_flight_times() {
        let calendar = from_file(&"tests/fixtures/calendar.ics".to_string()).unwrap();
        let next_event = get_next_event(&calendar).unwrap();
        let test_times = Some((
            chrono::NaiveDate::from_ymd_opt(2026, 08, 23)
                .unwrap()
                .and_hms_opt(19, 35, 22)
                .unwrap(),
            chrono::NaiveDate::from_ymd_opt(2030, 09, 08)
                .unwrap()
                .and_hms_opt(10, 58, 00)
                .unwrap(),
        ));

        let times = Some((
            get_start_ndt(next_event).unwrap(),
            get_end_ndt(next_event).unwrap(),
        ));
        assert_eq!(times, test_times);
    }
}
