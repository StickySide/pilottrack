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

        let username = env::var("UNUMBER").context("'UNUMBER' not found")?;
        let password = env::var("PASSWORD").context("'PASSWORD' not found")?;
        let url = env::var("URL").context("'URL' not found")?;

        Ok(Config {
            username,
            password,
            url,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loads_env() {
        dotenvy::from_filename("./tests/fixtures/.env").ok();
        assert_eq!(std::env::var("UNUMBER"), Ok(String::from("u123456")));
        assert_eq!(std::env::var("PASSWORD"), Ok(String::from("password")));
        assert_eq!(
            std::env::var("URL"),
            Ok(String::from("http://www.example.com"))
        )
    }
}
