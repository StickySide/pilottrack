use anyhow::Result;
use reqwest::header::USER_AGENT;
use std::env;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let username = env::var("USERNAME")?;
    let password = env::var("PASSWORD")?;
    let url = env::var("URL")?;

    let client = reqwest::blocking::Client::builder().build()?;

    println!("U: {}\nP: {}\nURL: {}", username, password, url);

    let result = client
        .get(&url)
        .basic_auth(username, Some(password))
        .header(USER_AGENT, "curl/8.5.0")
        .send()?;

    Ok(())
}
