use chrono::{ParseResult, prelude::*};

use crate::flight::LiveUpdate;
use serde_json::Value;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn from_file(filename: String) -> std::io::Result<String> {
    let file = std::fs::read_to_string(filename);
    file
}

pub fn get_live_update(
    dt: Option<chrono::NaiveDateTime>,
    callsign: Option<String>,
) -> anyhow::Result<LiveUpdate> {
    let data = match (dt, callsign) {
        (Some(dt), Some(callsign)) => from_url(dt, callsign)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Callsign or date time missing when trying to request live update from flight stats"
            ));
        }
    };

    let data: Value = serde_json::from_str(&data)?;

    let status = match data["data"]["status"]["status"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let estimated_departure = match chrono::DateTime::parse_from_rfc3339(
        &data["data"]["schedule"]["estimatedActualDeparture"].to_string(),
    ) {
        Ok(dt) => Some(dt.naive_local()),
        Err(e) => {
            eprintln!("Could not parse estimated_departure: {e}");
            None
        }
    };

    let estimated_arrival = match chrono::DateTime::parse_from_rfc3339(
        &data["data"]["schedule"]["estimatedActualArrival"].to_string(),
    ) {
        Ok(dt) => Some(dt.naive_local()),
        Err(e) => {
            eprintln!("Could not parse estimated_arrival: {e}");
            None
        }
    };

    Ok(LiveUpdate {
        status,
        estimated_departure,
        estimated_arrival,
    })

    // pub fn parse(data: String) -> anyhow::Result<FlightStats> {
    //     let data: serde_json::Value = serde_json::from_str(&data)?;

    //     let carrier_name: Option<String> = match data["data"]["resultHeader"]["carrier"]["name"].clone()
    //     {
    //         Value::String(x) => Some(x),
    //         _ => None,
    //     };

    //     let flight_number = match data["data"]["resultHeader"]["flightNumber"].clone() {
    //         Value::String(x) => Some(x),
    //         _ => None,
    //     };

    //     let status = match data["data"]["status"]["status"].clone() {
    //         Value::String(x) => Some(x),
    //         _ => None,
    //     };

    //     let error = match data["error"] {
    //         Value::Null => false,
    //         _ => true,
    //     };

    //     let stats = FlightStats {
    //         carrier_name,
    //         flight_number,
    //         status,
    //         error,
    //     };

    //     Ok(stats)
}
