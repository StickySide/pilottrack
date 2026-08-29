use chrono::prelude::*;
use serde_json::Value;

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
// }

#[cfg(test)]
mod tests {
    use super::*;
}
