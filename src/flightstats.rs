use std::fmt::Display;

use chrono::prelude::*;
use serde_json::Value;

#[derive(Default, Debug)]
pub struct FlightStats {
    pub error: bool,
    carrier_name: Option<String>,
    flight_number: Option<String>,
    status: Option<String>,
}

impl Display for FlightStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u = &"Unavailable";

        if self.error == true {
            let result = write!(f, "Error getting flight data");
            return result;
        }

        write!(
            f,
            "Carrier: {}\nFlight #: {}\nStatus: {}",
            self.carrier_name.clone().unwrap_or(u.to_string()),
            self.flight_number.clone().unwrap_or(u.to_string()),
            self.status.clone().unwrap_or(u.to_string()),
        )
    }
}

pub fn from_url(
    dt: chrono::NaiveDateTime,
    flight_number: String,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder().build()?;

    let url = format!(
        "https://www.flightstats.com/v2/api-next/flight-tracker/UA/{}/{}/{}/{}",
        flight_number,
        dt.year(),
        dt.month(),
        dt.day()
    );

    dbg!(&url);

    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "curl/8.5.0")
        .send()?
        .text()?;

    Ok(response)
}

pub fn from_file(filename: String) -> std::io::Result<String> {
    let file = std::fs::read_to_string(filename);
    file
}

pub fn parse(data: String) -> anyhow::Result<FlightStats> {
    let data: serde_json::Value = serde_json::from_str(&data)?;

    let carrier_name: Option<String> = match data["data"]["resultHeader"]["carrier"]["name"].clone()
    {
        Value::String(x) => Some(x),
        _ => None,
    };

    let flight_number = match data["data"]["resultHeader"]["flightNumber"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let status = match data["data"]["status"]["status"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let error = match data["error"] {
        Value::Null => false,
        _ => true,
    };

    let stats = FlightStats {
        carrier_name,
        flight_number,
        status,
        error,
    };

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_data() {
        let data = from_file(String::from("tests/fixtures/test_flight_stats.json")).unwrap();
        let stats = parse(data).unwrap();
        let test_stats = FlightStats {
            carrier_name: Some(String::from("United Airlines")),
            flight_number: Some(String::from("1469")),
            status: Some(String::from("Departed")),
            error: false,
        };
        assert_eq!(stats.carrier_name, test_stats.carrier_name);
        assert_eq!(stats.flight_number, test_stats.flight_number);
        assert_eq!(stats.status, test_stats.status);
    }
}
