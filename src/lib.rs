// This file contains the main Rust library code. It includes a simple "Hello World" function that can be called from Python.


use std::collections::HashMap;
use std::cell::RefCell;

mod encoding;
mod network;

use encoding::{CdcValue, CdcList, CdcDict};
use network::{Connection};
use uuid;

use std::env;

thread_local! {
    static GOM_CONNECTION: RefCell<Option<Connection>> = RefCell::new(None);
}

fn get_api_url() -> Option<String> {
    env::var("TOM_PYTHON_API_URL").ok()
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
                match Connection::init(&config.server_url, config.api_key) {
                    Ok(mut conn) => {
                        match conn.register(&config.interpreter_id, "zeiss_inspect_api_rust") {
                            Ok(_) => {
                                GOM_CONNECTION.with(|conn_cell| {
                                    *conn_cell.borrow_mut() = Some(conn);
                                });
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

/// Executes a GOM command with positional and keyword arguments.
///
/// This function executes a command in the GOM application, passing both positional arguments
/// and keyword arguments. The command execution happens asynchronously via the WebSocket connection.
///
/// # Arguments
/// * `command_name` - The name of the command to execute
/// * `args` - A list of positional arguments (as CdcValue items)
/// * `kwargs` - A map of keyword arguments (as CdcValue items)
///
/// # Returns
/// The result of the command execution, or an error if the command fails
pub fn execute_command(command_name: &str, args: CdcList, kwargs: CdcDict) -> Result<CdcValue, network::ConnectionError> {
    GOM_CONNECTION.with(|conn_cell| {
        let mut conn_guard = conn_cell.borrow_mut();
        
        if let Some(conn) = conn_guard.as_mut() {
            let mut params = HashMap::new();
            params.insert("command".to_string(), CdcValue::STRING(command_name.to_string()));
            params.insert("args".to_string(), CdcValue::LIST(args));
            params.insert("kwargs".to_string(), CdcValue::MAP(kwargs));
            
            conn.request(network::Request::COMMAND, params)
        } else {
            Err(network::ConnectionError::Request)
        }
    })
}

/// Translates the given text using the GOM application's translation system.
///
/// This function retrieves the translated version of a text string from the running ZEISS Inspect
/// application. If no translation is available, the original text is returned.
///
/// # Arguments
/// * `text` - The text to be translated
/// * `id` - Optional translation ID used by the GOM internal translation process
///
/// # Returns
/// The translated text, or the original text if translation fails or is unavailable
pub fn tr(text: &str, id: Option<&str>) -> String {
    GOM_CONNECTION.with(|conn_cell| {
        let mut conn_guard = conn_cell.borrow_mut();
        
        if let Some(conn) = conn_guard.as_mut() {
            let mut params = std::collections::HashMap::new();
            params.insert("text".to_string(), CdcValue::STRING(text.to_string()));
            params.insert(
                "id".to_string(),
                CdcValue::STRING(id.unwrap_or("").to_string()),
            );
            
            match conn.request(network::Request::TRANSLATE, params) {
                Ok(result) => {
                    if let CdcValue::MAP(mut result_map) = result {
                        if let Some(CdcValue::STRING(translation)) = result_map.remove("translation") {
                            return translation;
                        }
                    }
                }
                Err(_e) => {
                    log::warn!("Translation request failed, returning original text");
                }
            }
        } else {
            log::debug!("No GOM connection available, returning original text");
        }
        
        text.to_string()
    })
}


#[derive(Debug, Clone, PartialEq)]
struct CdcError {
    id: String,
    text: String,
    line: i64,
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

impl Item {
    /// Creates a new Item with the specified ID, category, and stage.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for this item
    /// * `category` - The category this item belongs to (default: 0)
    /// * `stage` - The stage this item is associated with (default: -1)
    pub fn new(id: String, category: i32, stage: i32) -> Self {
        Item { id, category, stage }
    }

    /// Retrieves the value of an attribute from this item.
    ///
    /// # Arguments
    /// * `key` - The name of the attribute to retrieve
    /// * `index` - Optional index for accessing array-like attributes
    pub fn get(&self, key: &str, index: Option<i64>) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("name".to_string(), CdcValue::STRING(key.to_string()));
                if let Some(idx) = index {
                    params.insert("index".to_string(), CdcValue::INTEGER(idx));
                }
                conn.request(network::Request::GET, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Retrieves all available tokens for this item.
    pub fn get_tokens(&self) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                conn.request(network::Request::TOKENS, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Filters this item using the provided expression.
    ///
    /// # Arguments
    /// * `expression` - The filter expression to apply
    /// * `condition` - Optional filter condition
    pub fn filter(&self, expression: &str, condition: Option<&str>) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("expression".to_string(), CdcValue::STRING(expression.to_string()));
                if let Some(cond) = condition {
                    params.insert("condition".to_string(), CdcValue::STRING(cond.to_string()));
                }
                conn.request(network::Request::FILTER, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Compares this item with another using the less-than operator.
    pub fn less_than(&self, other: &Item) -> Result<bool, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("other".to_string(), CdcValue::MAP(other.to_map()?));
                match conn.request(network::Request::LESS, params)? {
                    CdcValue::BOOL(result) => Ok(result),
                    _ => Err(network::ConnectionError::Request),
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Checks if this item equals another item.
    pub fn equals(&self, other: &Item) -> Result<bool, network::ConnectionError> {
        // Fast path: compare by ID and category for same items
        if self.category == other.category && self.id == other.id {
            return Ok(true);
        }
        
        // Server-side comparison for different items
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("other".to_string(), CdcValue::MAP(other.to_map()?));
                match conn.request(network::Request::EQUAL, params)? {
                    CdcValue::BOOL(result) => Ok(result),
                    _ => Err(network::ConnectionError::Request),
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Accesses an attribute of this item.
    ///
    /// # Arguments
    /// * `name` - The name of the attribute to access
    pub fn get_attr(&self, name: &str) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("name".to_string(), CdcValue::STRING(name.to_string()));
                params.insert("stage".to_string(), CdcValue::INTEGER(self.stage as i64));
                conn.request(network::Request::GETATTR, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Sets an attribute of this item.
    ///
    /// # Arguments
    /// * `name` - The name of the attribute to set
    /// * `value` - The value to set
    pub fn set_attr(&self, name: &str, value: CdcValue) -> Result<(), network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("name".to_string(), CdcValue::STRING(name.to_string()));
                params.insert("value".to_string(), value);
                conn.request(network::Request::SETATTR, params)?;
                Ok(())
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Accesses an item by key (indexing operator).
    ///
    /// # Arguments
    /// * `key` - The key to access
    pub fn get_item(&self, key: &str) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                params.insert("name".to_string(), CdcValue::STRING(key.to_string()));
                conn.request(network::Request::KEY, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Returns the length of this item.
    pub fn len(&self) -> Result<i64, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                match conn.request(network::Request::LEN, params)? {
                    CdcValue::INTEGER(len) => Ok(len),
                    _ => Err(network::ConnectionError::Request),
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Returns true if the item is empty (has no elements).
    pub fn is_empty(&self) -> Result<bool, network::ConnectionError> {
        self.len().map(|len| len == 0)
    }

    /// Gets the string representation of this item.
    pub fn repr(&self) -> Result<String, network::ConnectionError> {
        // Fast path for API items
        if self.id.starts_with("gom.") {
            return Ok(self.id.clone());
        }

        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("item".to_string(), CdcValue::MAP(self.to_map()?));
                match conn.request(network::Request::REPR, params)? {
                    CdcValue::STRING(repr) => Ok(repr),
                    _ => Err(network::ConnectionError::Request),
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Returns the documentation for this item.
    pub fn doc(&self) -> Result<String, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("object".to_string(), CdcValue::MAP(self.to_map()?));
                match conn.request(network::Request::DOC, params)? {
                    CdcValue::STRING(doc) => Ok(doc),
                    _ => Err(network::ConnectionError::Request),
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }

    /// Converts this Item to a CDC map for transmission.
    fn to_map(&self) -> Result<HashMap<String, CdcValue>, network::ConnectionError> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), CdcValue::STRING(self.id.clone()));
        map.insert("category".to_string(), CdcValue::INTEGER(self.category as i64));
        map.insert("stage".to_string(), CdcValue::INTEGER(self.stage as i64));
        Ok(map)
    }

    /// Creates a JSON representation of this item.
    pub fn to_json(&self) -> HashMap<String, CdcValue> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), CdcValue::STRING(self.id.clone()));
        map.insert("category".to_string(), CdcValue::INTEGER(self.category as i64));
        map.insert("stage".to_string(), CdcValue::INTEGER(self.stage as i64));
        map
    }

    /// Creates an API JSON representation of this item (for protocol messages).
    pub fn to_api_json(&self) -> HashMap<String, CdcValue> {
        let mut map = HashMap::new();
        map.insert("$type".to_string(), CdcValue::STRING("reference".to_string()));
        map.insert("id".to_string(), CdcValue::STRING(self.id.clone()));
        map.insert("category".to_string(), CdcValue::INTEGER(self.category as i64));
        map
    }

    /// Creates an Item from parameters (typically from server response).
    pub fn from_params(params: &HashMap<String, CdcValue>) -> Result<Self, network::ConnectionError> {
        let id = params
            .get("id")
            .and_then(|v| if let CdcValue::STRING(s) = v { Some(s.clone()) } else { None })
            .ok_or(network::ConnectionError::Request)?;
        
        let category = params
            .get("category")
            .and_then(|v| if let CdcValue::INTEGER(i) = v { Some(*i) } else { None })
            .unwrap_or(0) as i32;
        
        let stage = params
            .get("stage")
            .and_then(|v| if let CdcValue::INTEGER(i) = v { Some(*i) } else { None })
            .unwrap_or(-1) as i32;
        
        Ok(Item { id, category, stage })
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a Python slice object with start and stop values.
///
/// A Slice represents a portion of a sequence, defined by optional start and stop indices.
pub struct Slice {
    /// The start index of the slice (None if not specified).
    pub start: Option<i64>,
    /// The stop index of the slice (None if not specified).
    pub stop: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents an Indexable object from the GOM type system.
///
/// An Indexable wraps an item with an access token and size information,
/// allowing indexed access to item properties.
pub struct Indexable {
    /// The item being indexed.
    pub item: Item,
    /// The access token for this indexable object.
    pub token: String,
    /// The size of the indexable collection.
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a Trait object from the GOM type system.
///
/// A Trait is a generic type instance with an identifier and arguments.
/// It can hold positional and keyword arguments for parameterized type instantiation.
pub struct Trait {
    /// The type identifier for this trait.
    pub id: String,
    /// Positional arguments (list of values).
    pub args: CdcList,
    /// Keyword arguments (map of values).
    pub kwargs: CdcDict,
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
    // Before running this test, ensure that a WebSocket server is running at ws://localhost:3012 that can accept connections with the specified parameters.
    fn test_initialize_gom_connection() {

        // Set environment variable for testing
        std::env::set_var("TOM_PYTHON_API_URL", "ws://localhost:3012?apikey=test&interpreter_id=rust-test");
        
        // Call the initialization function
        initialize_gom_connection();
        
        // Check if connection was established (this is a simple check, in real test we'd verify more)
        // For now, just ensure no panic
    }

    #[test]
    fn test_tr_without_connection() {
        // Test that tr returns original text when no connection is available
        let result = tr("Hello World", None);
        assert_eq!(result, "Hello World");

        let result_with_id = tr("Test Text", Some("test_id"));
        assert_eq!(result_with_id, "Test Text");
    }
}