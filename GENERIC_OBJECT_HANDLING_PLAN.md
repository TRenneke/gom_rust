# Plan: Implementing Generic Object Handling in Rust Module

## Overview

This document outlines the implementation plan for adding generic object handling capabilities to the Rust GOM API module, mirroring the Python implementation. This includes support for:

- **Trait Objects** (dynamically registered types with lazy attribute resolution)
- **Generic Objects** (unspecialized objects without script type interfaces)
- **Data Arrays** (array/vector data containers)
- **Packages** (DataInterface::Package references)

---

## Implementation Progress

### Session 2 - Completion Summary (February 23, 2026)
**Status**: ✅ **Phase 3 COMPLETED**

Implemented the Type Registry System:
- Created new `src/types.rs` module with `TypeRegistry` struct
- Implemented thread-local type registry for managing dynamically registered types
- Added functions: `register_type()`, `is_type_registered()`, `get_type_name()`, `get_all_registered_types()`, `clear_type_cache()`, `clear_all_caches()`
- Added comprehensive unit tests for all registry functions
- Updated `src/lib.rs` to import and publicly export types module
- **Compilation test**: ✅ PASS - Code compiles without errors
- **Unit tests**: ✅ PASS - All 4 type registry tests pass

**Next Phase**: Phase 4 - Lazy Attribute Resolution

### Session 1 - Completion Summary (February 23, 2026)
**Status**: ✅ **Phase 1 & Phase 2 COMPLETED**

Implemented the foundational data structures and encoding/decoding layer:
- Added `Object`, `Array`, and `Package` struct definitions to `src/lib.rs`
- Added `CdcValue::OBJECT`, `CdcValue::ARRAY`, and `CdcValue::PACKAGE` variants to encoding enum
- Implemented `From<&CdcValue>` trait mapping for all three new types
- Added `expect_object()`, `expect_array()`, `expect_package()`, and `expect_trait()` methods
- Implemented full encoding logic for all three types (Object, Array, Package)
- Implemented full decoding logic for all three types
- Fixed recursive type issue by boxing `Array.project` and `Array.item` fields
- **Compilation test**: ✅ PASS - Code compiles without errors

---

## Current State Analysis

### What's Already Implemented
- ✅ `Trait` struct in lib.rs (represents generic type instances)
- ✅ `Trait` encoding/decoding in encoding.rs
- ✅ `CdcType::TRAIT`, `CdcType::OBJECT`, `CdcType::ARRAY`, `CdcType::PACKAGE` enum definitions
- ✅ Network request types for type operations (TYPE_GETATTR, TYPE_SETATTR, TYPE_CALL, etc.)
- ✅ Basic WebSocket connection and request handling
- ✅ `CdcValue::OBJECT` variant (discriminant 14) - COMPLETED
- ✅ `CdcValue::ARRAY` variant (discriminant 15) - COMPLETED
- ✅ `CdcValue::PACKAGE` variant (discriminant 16) - COMPLETED
- ✅ `Object` struct definition (generic object container) - COMPLETED
- ✅ `Array` struct definition (data array container) - COMPLETED
- ✅ `Package` struct definition (package reference container) - COMPLETED
- ✅ Encoding/decoding logic for Object, Array, Package types - COMPLETED
- ✅ Type mapping for CdcType::From<&CdcValue> - COMPLETED
- ✅ expect_trait, expect_object, expect_array, expect_package methods - COMPLETED
- ✅ `TypeRegistry` struct in types.rs - COMPLETED
- ✅ Thread-local type registry management - COMPLETED
- ✅ Public API functions for type registration - COMPLETED

### What's Missing
- ❌ Lazy attribute resolution infrastructure
- ❌ Support methods for accessing attributes on generic objects
- ❌ Integration of lazy-loaded attributes via network requests

---

## Implementation Plan

### Phase 1: Data Structures (Foundation) ✅ COMPLETED

#### Step 1.1: Add Missing Structs ✅ COMPLETED

**Location**: `src/lib.rs`

Add three new struct definitions alongside existing `Trait`:

```rust
/// Represents a generic object instance without specialized script type interface
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    /// The type identifier of this object
    pub type_id: String,
    /// String representation of the object
    pub repr: String,
    /// Object attributes as key-value pairs
    pub attributes: HashMap<String, CdcValue>,
}

/// Represents a data array container
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    /// The project this array belongs to
    pub project: Item,
    /// The item this array is part of
    pub item: Item,
    /// The key/token name for the array
    pub key: String,
    /// Index path for nested access
    pub index: Vec<i64>,
    /// Whether this is selected data
    pub selected: bool,
    /// Optional transformation data
    pub transformation: Option<Box<CdcValue>>,
}

/// Represents a DataInterface::Package reference
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    /// Package reference/handle
    pub reference: String,
    /// Package metadata
    pub metadata: CdcDict,
}
```

#### Step 1.2: Update CdcValue Enum ✅ COMPLETED

**Location**: `src/encoding.rs`

Add three new variants to the `CdcValue` enum:

```rust
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum CdcValue {
    // ... existing variants ...
    TRAIT(Trait) = 13,
    OBJECT(Object) = 14,          // NEW
    ARRAY(Array) = 15,             // NEW
    VEC2D(Vec2d) = 17,
    VEC3D(Vec3d) = 18,
    RESOURCE_ACCESS = 19,
    BLOB(Vec<u8>) = 20,
    PACKAGE(Package) = 16,         // NEW
}
```

#### Step 1.3: Update CdcValue Type Mapping ✅ COMPLETED

**Location**: `src/encoding.rs` - `impl From<&CdcValue> for CdcType`

Add match cases to map the new variants:

```rust
CdcValue::OBJECT(_) => CdcType::OBJECT,
CdcValue::ARRAY(_) => CdcType::ARRAY,
CdcValue::PACKAGE(_) => CdcType::PACKAGE,
```

#### Step 1.4: Add Expect Methods ✅ COMPLETED

**Location**: `src/encoding.rs` - `impl CdcValue`

Add new expect methods for the new types:

```rust
pub fn expect_object(self) -> Object {
    if let CdcValue::OBJECT(obj) = self { obj } 
    else { panic!("Expected OBJECT, found {:?}", self); }
}

pub fn expect_array(self) -> Array {
    if let CdcValue::ARRAY(arr) = self { arr } 
    else { panic!("Expected ARRAY, found {:?}", self); }
}

pub fn expect_package(self) -> Package {
    if let CdcValue::PACKAGE(pkg) = self { pkg } 
    else { panic!("Expected PACKAGE, found {:?}", self); }
}
```

---

### Phase 2: Encoding/Decoding Layer ✅ COMPLETED

#### Step 2.1: Implement Object Encoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn encode_value(&mut self, buffer: &mut Vec<u8>, value: &CdcValue)`

Add encoding logic for Object variant:

```rust
CdcValue::OBJECT(obj) => {
    // Type discriminant
    buffer.push(CdcType::OBJECT as u8);
    // Type ID (string)
    CdcEncoder::encode_string(buffer, &obj.type_id);
    // Repr (string)
    CdcEncoder::encode_string(buffer, &obj.repr);
    // Attributes count (i64)
    CdcEncoder::encode_number(buffer, obj.attributes.len() as i64);
    // Encode each attribute
    for (key, value) in &obj.attributes {
        CdcEncoder::encode_string(buffer, key);
        self.encode_value(buffer, value);
    }
}
```

#### Step 2.2: Implement Array Encoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn encode_value(&mut self, buffer: &mut Vec<u8>, value: &CdcValue)`

Add encoding logic for Array variant:

```rust
CdcValue::ARRAY(arr) => {
    buffer.push(CdcType::ARRAY as u8);
    // Encode project
    self.encode_value(buffer, &arr.project);
    // Encode item
    self.encode_value(buffer, &arr.item);
    // Encode key
    CdcEncoder::encode_string(buffer, &arr.key);
    // Encode index path
    CdcEncoder::encode_number(buffer, arr.index.len() as i64);
    for idx in &arr.index {
        CdcEncoder::encode_number(buffer, *idx);
    }
    // Encode selected flag
    buffer.push(if arr.selected { 1 } else { 0 });
    // Encode transformation (optional)
    match &arr.transformation {
        Some(trans) => {
            buffer.push(1);
            self.encode_value(buffer, trans);
        }
        None => buffer.push(0),
    }
}
```

#### Step 2.3: Implement Package Encoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn encode_value(&mut self, buffer: &mut Vec<u8>, value: &CdcValue)`

Add encoding logic for Package variant:

```rust
CdcValue::PACKAGE(pkg) => {
    buffer.push(CdcType::PACKAGE as u8);
    CdcEncoder::encode_string(buffer, &pkg.reference);
    CdcEncoder::encode_number(buffer, pkg.metadata.len() as i64);
    for (key, value) in &pkg.metadata {
        CdcEncoder::encode_string(buffer, key);
        self.encode_value(buffer, value);
    }
}
```

#### Step 2.4: Implement Object Decoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn decode(&mut self, buffer: &[u8], pos: &mut usize) -> Result<CdcValue, Box<dyn std::error::Error>>`

Add decoding logic in the match statement (around line 465):

```rust
x if x == CdcType::OBJECT as u8 => {
    let type_id = CdcEncoder::decode_string(buffer, pos)?;
    let repr = CdcEncoder::decode_string(buffer, pos)?;
    let attr_count = CdcEncoder::decode_number(buffer, pos)? as usize;
    
    let mut attributes = HashMap::new();
    for _ in 0..attr_count {
        let key = CdcEncoder::decode_string(buffer, pos)?;
        let value = self.decode(buffer, pos)?;
        attributes.insert(key, value);
    }
    
    Ok(CdcValue::OBJECT(Object { type_id, repr, attributes }))
}
```

#### Step 2.5: Implement Array Decoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn decode(&mut self, buffer: &[u8], pos: &mut usize) -> Result<CdcValue, Box<dyn std::error::Error>>`

Add decoding logic:

```rust
x if x == CdcType::ARRAY as u8 => {
    let project = self.decode(buffer, pos)?;
    let item = self.decode(buffer, pos)?;
    let key = CdcEncoder::decode_string(buffer, pos)?;
    
    let index_len = CdcEncoder::decode_number(buffer, pos)? as usize;
    let mut index = Vec::new();
    for _ in 0..index_len {
        index.push(CdcEncoder::decode_number(buffer, pos)?);
    }
    
    let selected = buffer[*pos] != 0;
    *pos += 1;
    
    let transformation = if buffer[*pos] != 0 {
        *pos += 1;
        Some(Box::new(self.decode(buffer, pos)?))
    } else {
        *pos += 1;
        None
    };
    
    Ok(CdcValue::ARRAY(Array { project, item, key, index, selected, transformation }))
}
```

#### Step 2.6: Implement Package Decoding ✅ COMPLETED

**Location**: `src/encoding.rs` - `fn decode(&mut self, buffer: &[u8], pos: &mut usize) -> Result<CdcValue, Box<dyn std::error::Error>>`

Add decoding logic:

```rust
x if x == CdcType::PACKAGE as u8 => {
    let reference = CdcEncoder::decode_string(buffer, pos)?;
    let metadata_count = CdcEncoder::decode_number(buffer, pos)? as usize;
    
    let mut metadata = HashMap::new();
    for _ in 0..metadata_count {
        let key = CdcEncoder::decode_string(buffer, pos)?;
        let value = self.decode(buffer, pos)?;
        metadata.insert(key, value);
    }
    
    Ok(CdcValue::PACKAGE(Package { reference, metadata }))
}
```

---

### Phase 3: Type Registry System ✅ COMPLETED

#### Step 3.1: Create Type Registry Module ✅ COMPLETED

**Location**: `src/types.rs` (new file)

Create a new module to manage dynamically registered types:

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use crate::encoding::{CdcValue, CdcDict, CdcList, Trait};

thread_local! {
    static TYPE_REGISTRY: RefCell<TypeRegistry> = RefCell::new(TypeRegistry::new());
}

pub struct TypeRegistry {
    /// Maps type ID -> type name
    registered_types: HashMap<String, String>,
    /// Maps type ID -> cached type instances
    cached_instances: HashMap<String, Vec<CdcValue>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        TypeRegistry {
            registered_types: HashMap::new(),
            cached_instances: HashMap::new(),
        }
    }
    
    /// Register a new type with the server
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
    
    /// Clear cached instances for a type
    pub fn clear_cache(&mut self, type_id: &str) {
        self.cached_instances.remove(type_id);
    }
    
    /// Clear all caches
    pub fn clear_all_caches(&mut self) {
        self.cached_instances.clear();
    }
}

pub fn register_type(type_id: String, type_name: String) {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow_mut().register_type(type_id, type_name);
    });
}

pub fn is_type_registered(type_id: &str) -> bool {
    TYPE_REGISTRY.with(|registry| {
        registry.borrow().is_registered(type_id)
    })
}
```

#### Step 3.2: Import and Export Type Registry ✅ COMPLETED

**Location**: `src/lib.rs`

Add module declaration and re-export:

```rust
mod types;
pub use types::{register_type, is_type_registered};
```

---

### Phase 4: Lazy Attribute Resolution

#### Step 4.1: Create Lazy Attribute Access Trait

**Location**: `src/lib.rs`

Define a trait for objects that support lazy attribute access:

```rust
/// Trait for objects that support lazy attribute resolution via network requests
pub trait LazyAttributeAccess {
    /// Get an attribute, fetching from server if necessary
    fn get_attribute(&self, name: &str) -> Result<CdcValue, network::ConnectionError>;
    
    /// Set an attribute on the server
    fn set_attribute(&self, name: &str, value: CdcValue) -> Result<(), network::ConnectionError>;
}
```

#### Step 4.2: Implement Lazy Access for Trait

**Location**: `src/lib.rs`

Implement lazy attribute resolution for Trait objects:

```rust
impl Trait {
    /// Get an attribute from this trait, fetching from server if not in kwargs
    pub fn get_attribute(&self, name: &str) -> Result<CdcValue, network::ConnectionError> {
        // First check if already in cached kwargs
        if let Some(value) = self.kwargs.get(name) {
            return Ok(value.clone());
        }
        
        // Fetch from server
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("type".to_string(), CdcValue::STRING(self.id.clone()));
                params.insert("args".to_string(), CdcValue::LIST(self.args.clone()));
                params.insert("kwargs".to_string(), CdcValue::MAP(self.kwargs.clone()));
                params.insert("name".to_string(), CdcValue::STRING(name.to_string()));
                
                conn.request(network::Request::TYPE_GETATTR, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
    
    /// Set an attribute on this trait
    pub fn set_attribute(&self, name: &str, value: CdcValue) -> Result<(), network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("type".to_string(), CdcValue::STRING(self.id.clone()));
                params.insert("args".to_string(), CdcValue::LIST(self.args.clone()));
                params.insert("kwargs".to_string(), CdcValue::MAP(self.kwargs.clone()));
                params.insert("name".to_string(), CdcValue::STRING(name.to_string()));
                params.insert("value".to_string(), value);
                
                conn.request(network::Request::TYPE_SETATTR, params)?;
                Ok(())
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
}
```

#### Step 4.3: Implement Lazy Access for Object

**Location**: `src/lib.rs`

Add similar methods to Object for attribute access:

```rust
impl Object {
    /// Get an attribute, serving from cache first
    pub fn get_attribute(&self, name: &str) -> Option<CdcValue> {
        self.attributes.get(name).cloned()
    }
    
    /// Update or add an attribute
    pub fn set_attribute(&mut self, name: String, value: CdcValue) {
        self.attributes.insert(name, value);
    }
}
```

---

### Phase 5: Array Data Access

#### Step 5.1: Implement Array Methods

**Location**: `src/lib.rs`

Add methods to Array for data access:

```rust
impl Array {
    /// Get the shape of this array from the server
    pub fn get_shape(&self) -> Result<Vec<i64>, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("data".to_string(), CdcValue::ARRAY(self.clone()));
                
                match conn.request(network::Request::DATA_SHAPE, params)? {
                    CdcValue::LIST(list) => {
                        Ok(list.into_iter()
                            .filter_map(|v| if let CdcValue::INTEGER(i) = v { Some(i) } else { None })
                            .collect())
                    }
                    _ => Err(network::ConnectionError::Request)
                }
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
    
    /// Get an attribute of this array (like dtype, units, etc.)
    pub fn get_attribute(&self, name: &str) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                let mut array_map = HashMap::new();
                array_map.insert("project".to_string(), self.project.clone());
                array_map.insert("item".to_string(), self.item.clone());
                array_map.insert("key".to_string(), CdcValue::STRING(self.key.clone()));
                
                params.insert("data".to_string(), CdcValue::MAP(array_map));
                params.insert("name".to_string(), CdcValue::STRING(name.to_string()));
                
                conn.request(network::Request::DATA_ATTR, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
    
    /// Get a single element at the given index
    pub fn get_element(&self, index: i64) -> Result<CdcValue, network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("data".to_string(), CdcValue::ARRAY(self.clone()));
                params.insert("key".to_string(), CdcValue::INTEGER(index));
                
                conn.request(network::Request::DATA_INDEX, params)
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
}
```

---

### Phase 6: Package Operations

#### Step 6.1: Implement Package Methods

**Location**: `src/lib.rs`

Add methods to Package for reference management:

```rust
impl Package {
    /// Get the value of a metadata field
    pub fn get_metadata(&self, key: &str) -> Option<CdcValue> {
        self.metadata.get(key).cloned()
    }
    
    /// Release this package reference on the server
    pub fn release(&self) -> Result<(), network::ConnectionError> {
        GOM_CONNECTION.with(|conn_cell| {
            let mut conn_guard = conn_cell.borrow_mut();
            if let Some(conn) = conn_guard.as_mut() {
                let mut params = HashMap::new();
                params.insert("reference".to_string(), CdcValue::STRING(self.reference.clone()));
                
                conn.request(network::Request::RELEASE, params)?;
                Ok(())
            } else {
                Err(network::ConnectionError::Request)
            }
        })
    }
}
```

---

### Phase 7: Import Updates and Integration

#### Step 7.1: Update Module Imports

**Location**: `src/lib.rs`

Update imports at the top of the file:

```rust
mod encoding;
mod network;
mod types;

use encoding::{CdcValue, CdcList, CdcDict, CdcEncoder, Object, Array, Package};
use network::Connection;
use types::{register_type, is_type_registered};
```

Also add Object, Array, Package to the public API:

```rust
pub use encoding::{Object, Array, Package};
```

---

### Phase 8: Error Handling Enhancements

#### Step 8.1: Add Error Cases

**Location**: `src/network.rs`

Ensure ConnectionError covers all necessary error cases:

```rust
#[derive(Debug)]
pub enum ConnectionError {
    Request,
    Serialization(String),
    Deserialization(String),
    TypeNotRegistered(String),
    InvalidOperation(String),
}
```

#### Step 8.2: Update Error Handling

**Location**: `src/encoding.rs` and throughout

Ensure all encoding/decoding operations properly return errors rather than panicking:

```rust
pub fn decode_string(buffer: &[u8], pos: &mut usize) 
    -> Result<String, Box<dyn std::error::Error>> {
    // Implementation with proper error handling
}
```

---

### Phase 9: Testing and Validation

#### Step 9.1: Unit Tests

**Location**: `src/lib.rs` or `tests/` directory

Create comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_object_creation() {
        let mut attrs = HashMap::new();
        attrs.insert("x".to_string(), CdcValue::INTEGER(42));
        
        let obj = Object {
            type_id: "TestType".to_string(),
            repr: "TestObject".to_string(),
            attributes: attrs,
        };
        
        assert_eq!(obj.get_attribute("x"), Some(CdcValue::INTEGER(42)));
    }
    
    #[test]
    fn test_array_creation() {
        let arr = Array {
            project: CdcValue::NONE,
            item: CdcValue::NONE,
            key: "data".to_string(),
            index: vec![],
            selected: false,
            transformation: None,
        };
        
        assert_eq!(arr.key, "data");
    }
    
    #[test]
    fn test_type_encoding_decoding() {
        // Test that Object/Array/Package encode and decode properly
    }
}
```

#### Step 9.2: Integration Tests

Create integration tests with the test server:

```rust
#[test]
fn test_lazy_attribute_access() {
    // Test that attributes are fetched correctly from server
}

#[test]
fn test_array_data_access() {
    // Test that array data can be accessed through network
}
```

#### Step 9.3: Test Client Updates

**Location**: `rust_testclient/src/main.rs`

Update test client to exercise new functionality:

```rust
fn main() {
    // Test Object creation and serialization
    // Test Array data access
    // Test Package references
    // Test lazy attribute resolution
}
```

---

### Phase 10: Documentation and Examples

#### Step 10.1: Add Documentation Comments

Throughout the codebase, ensure all public types and methods have comprehensive documentation:

```rust
/// Represents a generic object instance without specialized script type interface.
///
/// Objects are used when the GOM server sends instances of types that don't have
/// dedicated Rust representations. The attributes are fetched from the server via
/// lazy resolution.
pub struct Object { ... }
```

#### Step 10.2: Create Usage Examples

**Location**: `examples/` directory

Create example files showing how to use the new types:

```rust
// examples/generic_objects.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    zeiss_inspect_api_rust::initialize_gom_connection()?;
    
    // Example usage of Object, Array, Package
    Ok(())
}
```

---

## Implementation Order / Priority

**Recommended implementation order for minimal dependencies:**

1. **Phase 1** (Data Structures) - No dependencies
2. **Phase 2** (Encoding/Decoding) - Depends on Phase 1
3. **Phase 8** (Error Handling) - Parallel with Phase 2
4. **Phase 3** (Type Registry) - Depends on Phase 2
5. **Phase 4** (Lazy Attribute Resolution) - Depends on Phase 3
6. **Phase 5** (Array Data Access) - Depends on Phase 4
7. **Phase 6** (Package Operations) - Depends on Phase 4
8. **Phase 7** (Import Updates) - Depends on all previous
9. **Phase 9** (Testing) - Depends on Phase 7
10. **Phase 10** (Documentation) - Final pass

---

## Key Design Decisions

### 1. **Lazy Loading Strategy**
- Attributes are only fetched when explicitly requested
- Cache results locally to minimize network requests
- Provide cache invalidation methods for updates

### 2. **Error Handling**
- Use `Result<T, ConnectionError>` for all network operations
- Implement proper error conversion with context
- Don't panic in production code; use recoverable errors

### 3. **Memory Management**
- Use `HashMap` for attribute storage (similar to Python `dict`)
- Use `Vec` for index paths and arrays
- Box optional nested structures to control memory layout

### 4. **Thread Safety**
- Use `thread_local!` for connection and registry (existing pattern)
- Immutable borrows for read operations
- Mutable borrows for write operations

### 5. **Compatibility with Python**
- Mirror the Python `Types` class behavior
- Use same encoding format for binary protocol
- Support all same request types (TYPE_GETATTR, TYPE_SETATTR, etc.)

---

## Success Criteria

- ✅ All three new CdcValue variants encode/decode correctly
- ✅ Type registry tracks registered types
- ✅ Lazy attribute resolution fetches from server
- ✅ Array data can be accessed and shaped retrieved
- ✅ Package references can be created and released
- ✅ All unit tests pass
- ✅ Integration tests with test server pass
- ✅ Test client successfully exercises new functionality
- ✅ No memory leaks or unsafe behavior
- ✅ Code follows Rust idioms and conventions

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Serialization incompatibilities | Extensive testing against Python-generated messages |
| Memory leaks with circular references | Use Rust's ownership system; test with valgrind |
| Network latency impacting performance | Implement caching layer for repeated accesses |
| Type safety confusion | Strong typing with newtype patterns where needed |
| Encoding edge cases | Comprehensive encoding/decoding test suite |

---

## Timeline Estimate

- **Phase 1-2**: 2-3 hours (data structures & encoding)
- **Phase 3-4**: 2-3 hours (registry & lazy loading)
- **Phase 5-6**: 1-2 hours (array & package operations)
- **Phase 7-8**: 1 hour (integration & error handling)
- **Phase 9-10**: 2-3 hours (testing & documentation)

**Total: ~10-14 hours of implementation**
