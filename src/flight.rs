// use std::fmt::Display;

use chrono::NaiveDateTime;

#[allow(dead_code)]
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
    pub delay_status: Option<String>,
    pub departure_delay: Option<u64>,
    pub arrival_delay: Option<u64>,
    pub estimated_departure: Option<chrono::NaiveDateTime>,
    pub estimated_arrival: Option<chrono::NaiveDateTime>,
    pub errors: Vec<String>,
}

impl Flight {
    pub fn live_update(&mut self, lu: LiveUpdate) {
        self.status = lu.status;
        self.delay_status = lu.delay_status;
        self.departure_delay = lu.departure_delay;
        self.arrival_delay = lu.arrival_delay;
        self.estimated_departure = lu.estimated_departure;
        self.estimated_arrival = lu.estimated_arrival;
        self.errors = lu.errors;

        // Fill missing data with parsed data from flightstats
        if let None = self.departure {
            self.departure = lu.departure_iata
        }

        if let None = self.arrival {
            self.arrival = lu.arrival_aita
        }
    }
}

#[derive(Debug, Default)]
pub struct LiveUpdate {
    pub status: Option<String>,
    pub delay_status: Option<String>,
    pub departure_delay: Option<u64>,
    pub arrival_delay: Option<u64>,
    pub estimated_departure: Option<NaiveDateTime>,
    pub estimated_arrival: Option<NaiveDateTime>,
    pub departure_iata: Option<String>,
    pub arrival_aita: Option<String>,
    pub errors: Vec<String>,
}

// impl Display for Flight {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let u = String::from("Unavailable");

//         write!(
//             f,
//             "Flight #: {}\nPlanned Route: {} -> {}\nScheduled departure: {}\nScheduled Arrival: {}\n\
//             Status: {}\nEstimated/Actual departure: {}\nEstimated/Actual arrival: {}",
//             self.flight_number.clone().unwrap_or(u.to_string()),
//             self.departure.clone().unwrap_or(u.to_owned()),
//             self.arrival.clone().unwrap_or(u.to_owned()),
//             self.scheduled_departure
//                 .map_or(u.to_owned(), |d| d.to_string()),
//             self.scheduled_arrival
//                 .map_or(u.to_owned(), |d| d.to_string()),
//             self.status.clone().unwrap_or(u.to_owned()),
//             self.estimated_departure
//                 .map_or(u.to_owned(), |d| d.to_string()),
//             self.estimated_arrival
//                 .map_or(u.to_owned(), |d| d.to_string()),
//         )
//     }
// }
