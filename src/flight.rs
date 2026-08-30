use std::fmt::Display;

use chrono::NaiveDateTime;

#[derive(Debug, Default)]
pub struct Flight {
    // This section is for calendar data
    pub flight_number: Option<String>,
    pub departure: Option<String>,
    pub arrival: Option<String>,
    pub scheduled_departure: Option<chrono::NaiveDateTime>,
    pub scheduled_arrival: Option<chrono::NaiveDateTime>,
    // This section is for flight stats data
    pub status: Option<String>,
    pub estimated_departure: Option<chrono::NaiveDateTime>,
    pub estimated_arrival: Option<chrono::NaiveDateTime>,
}

impl Flight {
    pub fn live_update(&mut self, lu: LiveUpdate) {
        self.status = lu.status;
        self.estimated_departure = lu.estimated_departure;
        self.estimated_arrival = lu.estimated_arrival;
    }

    pub fn callsign(&self) -> Option<String> {
        match &self.flight_number {
            Some(n) => Some(format!("UAL{n}")),
            None => None,
        }
    }
}

impl Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u = String::from("Unavailable");

        write!(
            f,
            "Flight #: {}\nPlanned Route: {} -> {}\nScheduled departure: {}\nScheduled Arrival: {}\n\
            Status: {}\nEstimated deptarture: {}\nEstimated arrival: {}",
            self.flight_number.clone().unwrap_or(u.to_string()),
            self.departure.clone().unwrap_or(u.to_owned()),
            self.arrival.clone().unwrap_or(u.to_owned()),
            self.scheduled_departure
                .map_or(u.to_owned(), |d| d.to_string()),
            self.scheduled_arrival
                .map_or(u.to_owned(), |d| d.to_string()),
            self.status.clone().unwrap_or(u.to_owned()),
            self.estimated_departure
                .map_or(u.to_owned(), |d| d.to_string()),
            self.scheduled_arrival
                .map_or(u.to_owned(), |d| d.to_string()),
        )
    }
}

#[derive(Debug, Default)]
pub struct LiveUpdate {
    pub status: Option<String>,
    pub estimated_departure: Option<NaiveDateTime>,
    pub estimated_arrival: Option<NaiveDateTime>,
}
