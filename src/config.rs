// src/config.rs

use dotenv::dotenv;
use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub redis_url: String,
    pub token: Option<String>,
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

        let redis_url = match env::var("REDIS_URL") {
            Ok(url) => url,
            Err(_) => {
                eprintln!("REDIS_URL not found, please set REDIS_URL variable in .env file");
                std::process::exit(1);
            }
        };

        let token = match env::var("TOKEN") {
            Ok(token) => Some(token),
            Err(_) => {
                eprintln!("Warning: Server is running without a token. Please set TOKEN variable in .env file to secure the server");
                "".to_string();
                None
            }
        };

        AppConfig {
            server_port,
            redis_url,
            token,
        }
    }
}
