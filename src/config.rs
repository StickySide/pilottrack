use anyhow::{Context, Result};
use std::env;

pub struct Config {
    pub username: String,
    pub password: String,
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let username = env::var("UNUMBER").context("Username not found")?;
        let password = env::var("PASSWORD").context("Password not found")?;
        let url = env::var("URL").context("URL not found")?;

        Ok(Config {
            username,
            password,
            url,
        })
    }
}
