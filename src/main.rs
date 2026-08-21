use anyhow::Result;
use std::env;
fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let username = env::var("USERNAME")?;
    let password = env::var("PASSWORD")?;

    println!("Username: {}", username);
    println!("Password: {}", password);
    Ok(())
}
