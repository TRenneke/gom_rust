use std::collections::HashMap;
use std::cell::RefCell;
use crate::encoding::CdcValue;

thread_local! {
    static TYPE_REGISTRY: RefCell<TypeRegistry> = RefCell::new(TypeRegistry::new());
}

/// Manages dynamically registered types from the GOM server
pub struct TypeRegistry {
    /// Maps type ID -> type name
    registered_types: HashMap<String, String>,
    /// Maps type ID -> cached type instances
    cached_instances: HashMap<String, Vec<CdcValue>>,
}

impl TypeRegistry {
    /// Create a new empty type registry
    pub fn new() -> Self {
        TypeRegistry {
            registered_types: HashMap::new(),
            cached_instances: HashMap::new(),
        }
    }
    
    /// Register a new type with the registry
    pub fn register_type(&mut self, type_id: String, type_name: String) {
        self.registered_types.insert(type_id, type_name);
    }
    
    /// Check if a type is registered
    pub fn is_registered(&self, type_id: &str) -> bool {
        self.registered_types.contains_key(type_id)
    }
    
    /// Get the name of a registered type
    pub fn get_type_name(&self, type_id: &str) -> Option<&str> {
        self.registered_types.get(type_id).map(|s| s.as_str())
    }
    
    /// Get all registered types
    pub fn get_all_types(&self) -> Vec<(String, String)> {
        self.registered_types
            .iter()
            .map(|(id, name)| (id.clone(), name.clone()))
            .collect()
    }
    
    /// Clear cached instances for a type
    pub fn clear_cache(&mut self, type_id: &str) {
        self.cached_instances.remove(type_id);
    }
    
    /// Clear all caches
    pub fn clear_all_caches(&mut self) {
        self.cached_instances.clear();
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Register a new type with the global registry
pub fn register_type(type_id: String, type_name: String) {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow_mut().register_type(type_id, type_name);
    });
}

/// Check if a type is registered in the global registry
pub fn is_type_registered(type_id: &str) -> bool {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow().is_registered(type_id)
    })
}

/// Get the name of a registered type from the global registry
pub fn get_type_name(type_id: &str) -> Option<String> {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow().get_type_name(type_id).map(|s| s.to_string())
    })
}

/// Get all registered types from the global registry
pub fn get_all_registered_types() -> Vec<(String, String)> {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow().get_all_types()
    })
}

/// Clear the cache for a specific type
pub fn clear_type_cache(type_id: &str) {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow_mut().clear_cache(type_id);
    });
}

/// Clear all type caches
pub fn clear_all_caches() {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow_mut().clear_all_caches();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_type() {
        let mut registry = TypeRegistry::new();
        registry.register_type("test_id".to_string(), "TestType".to_string());
        
        assert!(registry.is_registered("test_id"));
        assert_eq!(registry.get_type_name("test_id"), Some("TestType"));
    }

    #[test]
    fn test_unregistered_type() {
        let registry = TypeRegistry::new();
        
        assert!(!registry.is_registered("unknown"));
        assert_eq!(registry.get_type_name("unknown"), None);
    }

    #[test]
    fn test_global_register_type() {
        register_type("global_test".to_string(), "GlobalTestType".to_string());
        
        assert!(is_type_registered("global_test"));
        assert_eq!(get_type_name("global_test"), Some("GlobalTestType".to_string()));
    }

    #[test]
    fn test_clear_cache() {
        let mut registry = TypeRegistry::new();
        registry.register_type("test_cache".to_string(), "CacheTestType".to_string());
        
        registry.clear_cache("test_cache");
        // Verify that clear_cache doesn't fail
        assert!(registry.is_registered("test_cache"));
    }
}
