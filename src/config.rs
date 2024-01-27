// src/config.rs

use dotenv::dotenv;
use std::env;

/// Application configuration
pub struct AppConfig {
    pub server_port: u16,
    pub redis_url: String,
}

impl AppConfig {
    /// Load and return the application configuration.
    pub fn new() -> Self {
        // Load environment variables from .env file
        dotenv().ok();

        // Retrieve each configuration variable
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a number");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

        AppConfig {
            server_port,
            redis_url,
        }
    }
}
