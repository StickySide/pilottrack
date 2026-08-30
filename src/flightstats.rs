use anyhow::Context;
use chrono::prelude::*;

use crate::flight::LiveUpdate;
use serde_json::Value;

#[allow(dead_code)]
pub fn from_url(
    dt: chrono::NaiveDateTime,
    flight_number: &String,
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
    callsign: &Option<String>,
) -> anyhow::Result<LiveUpdate> {
    let data = match (dt, callsign) {
        (Some(dt), Some(callsign)) => from_url(dt, callsign)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Callsign or date time missing when trying to request live update from flight stats"
            ));
        }
    };

    fn parse_optional_naive_datetime(value: &Value) -> anyhow::Result<chrono::NaiveDateTime> {
        let s = value.as_str().ok_or(anyhow::anyhow!(
            "Could not convert JSON time Value into str"
        ))?;

        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
            .context("Could not parse string into NaiveDateTime")
    }

    let mut live_update = LiveUpdate::default();

    let data: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            live_update.errors.push(format!(
                "Could not parse raw JSON data into usable Value:{e}"
            ));
            return Ok(live_update);
        }
    };

    let status = match data["data"]["status"]["status"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let estimated_departure = match parse_optional_naive_datetime(
        &data["data"]["schedule"]["estimatedActualDeparture"],
    ) {
        Ok(dt) => Some(dt),
        Err(e) => {
            live_update
                .errors
                .push(format!("Could not parse estimated departure time: {e}"));
            None
        }
    };

    let estimated_arrival =
        match parse_optional_naive_datetime(&data["data"]["schedule"]["estimatedActualArrival"]) {
            Ok(dt) => Some(dt),
            Err(e) => {
                live_update
                    .errors
                    .push(format!("Could not parse estimated arrival time: {e}"));
                None
            }
        };

    live_update.status = status;
    live_update.estimated_departure = estimated_departure;
    live_update.estimated_arrival = estimated_arrival;
    Ok(live_update)
}

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
