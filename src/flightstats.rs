use std::time::Duration;

use anyhow::Context;
use chrono::prelude::*;

use crate::flight::{
    LiveUpdate,
    TimeKind::{Actual, Estimated},
};
use serde_json::Value;

pub fn from_url(
    dt: &chrono::NaiveDateTime,
    flight_number: &String,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!(
        "https://www.flightstats.com/v2/api-next/flight-tracker/UA/{}/{}/{}/{}",
        flight_number,
        dt.year(),
        dt.month(),
        dt.day()
    );

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

fn parse_optional_naive_datetime(value: &Value) -> anyhow::Result<chrono::NaiveDateTime> {
    let s = value.as_str().ok_or(anyhow::anyhow!(
        "Could not convert JSON time Value into str"
    ))?;

    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
        .context("Could not parse string into NaiveDateTime")
}

pub fn get_live_update(
    departure_time: &Option<chrono::NaiveDateTime>,
    callsign: &Option<String>,
) -> anyhow::Result<LiveUpdate> {
    let data = match (departure_time, callsign) {
        (Some(dt), Some(callsign)) => from_url(dt, callsign)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Callsign or date time missing when trying to request live update from flight stats"
            ));
        }
    };

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

    let status = data["data"]["status"]["status"].as_str().map(String::from);

    let delay_status = data["data"]["status"]["delayStatus"]["wording"]
        .as_str()
        .map(String::from);

    let departure_delay = match data["data"]["status"]["delay"]["departure"]["minutes"].clone() {
        Value::Number(x) => x.as_u64(),
        _ => None,
    };

    let arrival_delay = match data["data"]["status"]["delay"]["arrival"]["minutes"].clone() {
        Value::Number(x) => x.as_u64(),
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

    let departure_iata = match data["data"]["departureAirport"]["iata"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let arrival_iata = match data["data"]["arrivalAirport"]["iata"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let departure_time_kind =
        match data["data"]["schedule"]["estimatedActualDepartureTitle"].clone() {
            Value::String(x) if x == "Actual" => Some(Actual),
            Value::String(x) if x == "Estimated" => Some(Estimated),
            _ => None,
        };

    let arrival_time_kind = match data["data"]["schedule"]["estimatedActualArrivalTitle"].clone() {
        Value::String(x) if x == "Actual" => Some(Actual),
        Value::String(x) if x == "Estimated" => Some(Estimated),
        _ => None,
    };

    live_update.status = status;
    live_update.delay_status = delay_status;
    live_update.departure_delay = departure_delay;
    live_update.arrival_delay = arrival_delay;
    live_update.estimated_departure = estimated_departure;
    live_update.departure_time_kind = departure_time_kind;
    live_update.arrival_time_kind = arrival_time_kind;
    live_update.estimated_arrival = estimated_arrival;
    live_update.departure_iata = departure_iata;
    live_update.arrival_aita = arrival_iata;

    Ok(live_update)
}
