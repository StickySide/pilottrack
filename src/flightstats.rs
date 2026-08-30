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

fn parse_optional_naive_datetime(value: &Value) -> Option<chrono::NaiveDateTime> {
    let s = value.as_str()?;
    match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
        Ok(dt) => Some(dt),
        Err(e) => {
            eprintln!("Unable to parse estimated_departure into DateTime: Chrono Parse Error: {e}");
            None
        }
    }
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

    let mut live_update = LiveUpdate::default();

    let data: Value = serde_json::from_str(&data)?;

    let status = match data["data"]["status"]["status"].clone() {
        Value::String(x) => Some(x),
        _ => None,
    };

    let estimated_departure =
        parse_optional_naive_datetime(&data["data"]["schedule"]["estimatedActualDeparture"]);
    let estimated_arrival =
        parse_optional_naive_datetime(&data["data"]["schedule"]["estimatedActualArrival"]);

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
