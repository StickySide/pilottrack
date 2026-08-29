use std::fmt::Display;

#[derive(Debug)]
pub struct Flight {
    pub number: Option<String>,
    pub scheduled_departure: Option<chrono::NaiveDateTime>,
    pub scheduled_arrival: Option<chrono::NaiveDateTime>,
}

impl Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u = &"Unavailable";

        write!(
            f,
            "Flight #: {}\nScheduled departure: {}\nScheduled Arrival: {}",
            self.number.clone().unwrap_or(u.to_string()),
            self.scheduled_departure
                .map_or(u.to_string(), |d| d.to_string()),
            self.scheduled_arrival
                .map_or(u.to_string(), |d| d.to_string()),
        )
    }
}
