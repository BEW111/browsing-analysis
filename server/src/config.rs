use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub frontend_url: String,
    pub extension_url: String,
    pub server_address: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "0.0.0.0:5173".to_string());
    let extension_url = env::var("EXTENSION_URL")?;
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    Ok(Config {
        database_url,
        frontend_url,
        extension_url,
        server_address,
    })
}
