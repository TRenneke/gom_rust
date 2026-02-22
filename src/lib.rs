// This file contains the main Rust library code. It includes a simple "Hello World" function that can be called from Python.


use std::collections::HashMap;
use std::sync::Mutex;

mod encoding;
mod network;

use encoding::{CdcValue, CdcDict};
use network::{Request, Conntection, ConnectionError};
use uuid;

use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref GOM_CONNECTION: Mutex<Option<Conntection>> = Mutex::new(None);
}

fn get_api_url() -> Option<String> {
    env::var("TOM_PYTHON_API_URL").ok()
}

#[derive(Debug, Clone)]
struct ConnectionConfig {
    server_url: String,
    api_key: String,
    interpreter_id: String,
    strip_tracebacks: bool,
}

fn parse_connection_config(api_url: &str) -> Result<ConnectionConfig, Box<dyn std::error::Error>> {
    let server_url = api_url.to_string();
    
    // Parse query parameters manually
    let query_start = api_url.find('?');
    let mut api_key = String::new();
    let mut interpreter_id = uuid::Uuid::new_v4().to_string();
    let mut strip_tracebacks = true;
    
    if let Some(query_start) = query_start {
        let query = &api_url[query_start + 1..];
        for pair in query.split('&') {
            let mut parts = pair.split('=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    "apikey" => api_key = value.to_string(),
                    "interpreter_id" => interpreter_id = value.to_string(),
                    "strip_tracebacks" => strip_tracebacks = value == "1",
                    _ => {}
                }
            }
        }
    }
    
    Ok(ConnectionConfig {
        server_url,
        api_key,
        interpreter_id,
        strip_tracebacks,
    })
}

pub fn initialize_gom_connection() {
    if let Some(api_url) = get_api_url() {
        match parse_connection_config(&api_url) {
            Ok(config) => {
                match Conntection::init(&config.server_url, config.api_key) {
                    Ok(mut conn) => {
                        match conn.register(&config.interpreter_id, "zeiss_inspect_api_rust") {
                            Ok(_) => {
                                *GOM_CONNECTION.lock().unwrap() = Some(conn);
                                log::info!("GOM connection initialized successfully");
                            }
                            Err(e) => log::error!("Failed to register interpreter: {:?}", e),
                        }
                    }
                    Err(e) => log::error!("Failed to initialize connection: {:?}", e),
                }
            }
            Err(e) => log::error!("Failed to parse connection config: {:?}", e),
        }
    } else {
        log::info!("No TOM_PYTHON_API_URL set, skipping GOM connection");
    }
}


#[derive(Debug, Clone, PartialEq)]
struct Command{
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Vec3d{
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Vec2d{
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents an item in the GOM application's item space.
///
/// An Item has a unique ID, belongs to a category, and is associated with a stage.
/// It provides methods for accessing and manipulating item attributes, filtering data,
/// and communicating with the server via WebSocket requests.
pub struct Item {
    /// The unique identifier of the item.
    pub id: String,
    /// The category this item belongs to.
    pub category: i32,
    /// The stage this item is associated with.
    pub stage: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_connection_config() {
        // Test with all parameters
        let api_url = "ws://localhost:41000?apikey=656bd8a17823f8e54bd2&interpreter_id=abc123&strip_tracebacks=1";
        let config = parse_connection_config(api_url).unwrap();
        assert_eq!(config.server_url, api_url);
        assert_eq!(config.api_key, "656bd8a17823f8e54bd2");
        assert_eq!(config.interpreter_id, "abc123");
        assert_eq!(config.strip_tracebacks, true);

        // Test with strip_tracebacks=0
        let api_url2 = "ws://localhost:41000?apikey=key&strip_tracebacks=0";
        let config2 = parse_connection_config(api_url2).unwrap();
        assert_eq!(config2.api_key, "key");
        assert_eq!(config2.strip_tracebacks, false);

        // Test with no query parameters
        let api_url3 = "ws://localhost:41000";
        let config3 = parse_connection_config(api_url3).unwrap();
        assert_eq!(config3.server_url, api_url3);
        assert_eq!(config3.api_key, "");
        assert_eq!(config3.strip_tracebacks, true);
        // interpreter_id should be generated, so not empty
        assert!(!config3.interpreter_id.is_empty());
    }
    #[test]
    fn test_initialize_gom_connection() {
        // Set environment variable for testing
        std::env::set_var("TOM_PYTHON_API_URL", "ws://localhost:3012?apikey=test&interpreter_id=rust-test");
        
        // Call the initialization function
        initialize_gom_connection();
        
        // Check if connection was established (this is a simple check, in real test we'd verify more)
        // For now, just ensure no panic
    }
}