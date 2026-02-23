use crate::{Vec2d, Vec3d, Command, Item, Slice, Indexable, Trait, CdcError, Object, Array, Package};
use std::{collections::HashMap, fmt};


/// Mirror constants from the Python JsonEncoder
const TYPE_DEFINITION_KEY: &str = "__TOM_TYPE_DEFINITION__";
const TYPE_BLOB: &str = "Tom::GScript::Blob";
const TYPE_CALLABLE: &str = "Tom::GScript::Callable";
const TYPE_COMMAND: &str = "Tom::GScript::Command";
const TYPE_ARRAY: &str = "Tom::GScript::Array";
const TYPE_INDEXABLE: &str = "Tom::GScript::Indexable";
const TYPE_ITEM: &str = "Tom::GScript::Item";
const TYPE_OBJECT: &str = "Tom::GScript::Object";
const TYPE_RESOURCE_ACCESS: &str = "Tom::GScript::ResourceAccess";
const TYPE_PACKAGE: &str = "Tom::DataInterface::Package";
const TYPE_SLICE: &str = "Tom::ScriptTypeInterface::Slice";
const TYPE_VEC2D: &str = "Tom::Vec2d";
const TYPE_VEC3D: &str = "Tom::Vec3d";


type CdcCallable = fn(CdcList, CdcDict) -> CdcValue;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CdcType {
    NONE = 0,
    BOOLEAN = 1,
    INTEGER = 2,
    FLOAT = 3,
    STRING = 4,
    LIST = 5,
    MAP = 6,
    SLICE = 7,
    ITEM = 8,
    INDEXABLE = 9,
    COMMAND = 10,
    CALLABLE = 11,
    ERROR = 12,
    TRAIT = 13,
    OBJECT = 14,
    ARRAY = 15,
    PACKAGE = 16,
    VEC2D = 17,
    VEC3D = 18,
    RESOURCE_ACCESS = 19,
    BLOB = 20,
}
impl From<&CdcValue> for CdcType {
    fn from(value: &CdcValue) -> Self {
        match value {
            CdcValue::NONE => CdcType::NONE,
            CdcValue::BOOL(_) => CdcType::BOOLEAN,
            CdcValue::INTEGER(_) => CdcType::INTEGER,
            CdcValue::FLOAT(_) => CdcType::FLOAT,
            CdcValue::STRING(_) => CdcType::STRING,
            CdcValue::LIST(_) => CdcType::LIST,
            CdcValue::MAP(_) => CdcType::MAP,
            CdcValue::SLICE(_) => CdcType::SLICE,
            CdcValue::ITEM(_) => CdcType::ITEM,
            CdcValue::INDEXABLE(_) => CdcType::INDEXABLE,
            CdcValue::COMMAND(_) => CdcType::COMMAND,
            CdcValue::CALLABLE(_) => CdcType::CALLABLE,
            CdcValue::ERROR(_) => CdcType::ERROR,
            CdcValue::TRAIT(_) => CdcType::TRAIT,
            CdcValue::OBJECT(_) => CdcType::OBJECT,
            CdcValue::ARRAY(_) => CdcType::ARRAY,
            CdcValue::PACKAGE(_) => CdcType::PACKAGE,
            CdcValue::VEC2D(_) => CdcType::VEC2D,
            CdcValue::VEC3D(_) => CdcType::VEC3D,
            CdcValue::RESOURCE_ACCESS => CdcType::RESOURCE_ACCESS,
            CdcValue::BLOB(_) => CdcType::BLOB,
        }
    }
}

pub type CdcDict = std::collections::HashMap<String, CdcValue>;
pub type CdcList = Vec<CdcValue>;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum CdcValue{
    NONE = 0,
    BOOL(bool) = 1,
    INTEGER(i64) = 2,
    FLOAT(f64) = 3,
    STRING(String) = 4,
    LIST(CdcList) = 5,
    MAP(CdcDict) = 6,
    SLICE(Slice) = 7,
    ITEM(Item) = 8,
    INDEXABLE(Indexable) = 9,
    COMMAND(Command) = 10,
    CALLABLE(CdcCallable) = 11,
    ERROR(CdcError) = 12,
    TRAIT(Trait) = 13,
    OBJECT(Object) = 14,
    ARRAY(Array) = 15,
    PACKAGE(Package) = 16,
    VEC2D(Vec2d) = 17,
    VEC3D(Vec3d) = 18,
    RESOURCE_ACCESS = 19,
    BLOB(Vec<u8>) = 20,
}
impl CdcValue {
    pub fn expect_bool(self) -> bool {
        if let CdcValue::BOOL(b) = self {b} else {panic!("Expected BOOL, found {:?}", self);}
    }
    pub fn expect_int(self) -> i64 {
        if let CdcValue::INTEGER(b) = self {b} else {panic!("Expected INTEGER, found {:?}", self);}
    }
    pub fn expect_float(self) -> f64 {
        if let CdcValue::FLOAT(b) = self {b} else {panic!("Expected FLOAT, found {:?}", self);}
    }
    pub fn expect_string(self) -> String {
        if let CdcValue::STRING(b) = self {b} else {panic!("Expected STRING, found {:?}", self);}
    }
    pub fn expect_list(self) -> CdcList {
        if let CdcValue::LIST(b) = self {b} else {panic!("Expected List, found {:?}", self);}
    }
    pub fn expect_map(self) -> CdcDict {
        if let CdcValue::MAP(b) = self {b} else {panic!("Expected MAP, found {:?}", self);}
    }
    pub fn expect_callable(self) -> CdcCallable {
        if let CdcValue::CALLABLE(b) = self {b} else {panic!("Expected CALLABLE, found {:?}", self);}
    }
    pub fn expect_vec2d(self) -> Vec2d {
        if let CdcValue::VEC2D(b) = self {b} else {panic!("Expected VEC2D, found {:?}", self);}
    }
    pub fn expect_vec3d(self) -> Vec3d {
        if let CdcValue::VEC3D(b) = self {b} else {panic!("Expected VEC3D, found {:?}", self);}
    }
    pub fn expect_command(self) -> Command {
        if let CdcValue::COMMAND(b) = self {b} else {panic!("Expected COMMAND, found {:?}", self);}
    }
    pub fn expect_blob(self) -> Vec<u8> {
        if let CdcValue::BLOB(b) = self {b} else {panic!("Expected BLOB, found {:?}", self);}
    }
    pub fn expect_error(self) -> CdcError {
        if let CdcValue::ERROR(b) = self {b} else {panic!("Expected ERROR, found {:?}", self);}
    }
    pub fn expect_item(self) -> Item {
        if let CdcValue::ITEM(b) = self {b} else {panic!("Expected ITEM, found {:?}", self);}
    }
    pub fn expect_slice(self) -> Slice {
        if let CdcValue::SLICE(b) = self {b} else {panic!("Expected SLICE, found {:?}", self);}
    }
    pub fn expect_indexable(self) -> Indexable {
        if let CdcValue::INDEXABLE(b) = self {b} else {panic!("Expected INDEXABLE, found {:?}", self);}
    }
    pub fn expect_trait(self) -> Trait {
        if let CdcValue::TRAIT(b) = self {b} else {panic!("Expected TRAIT, found {:?}", self);}
    }
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
}


impl CdcValue {
    fn discriminant(&self) -> u8 {
        // According to https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.access-memory
        // "If the enumeration specifies a primitive representation, 
        // then the discriminant may be reliably accessed via unsafe pointer casting."
        // This is the case here.
        unsafe { *(self as *const Self as *const u8) }
    }
}
pub struct CdcEncoder{
    registeredc_callables: HashMap<u64, fn(CdcList, CdcDict) -> CdcValue>,

}
#[derive(Debug, Clone)]
pub enum DecodeError {
    MissingData,
    UnknownType,
    MissingFunction,
}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::MissingData => write!(f, "The bytes buffer ended unexpectedly while trying to decode a value"),
            DecodeError::UnknownType => write!(f, "Unknown type discriminant encountered during decoding"),
            DecodeError::MissingFunction => write!(f, "Function pointer not found in registered callables"),
        }
    }
}

impl CdcEncoder{
    pub fn new() -> Self{
        CdcEncoder{
            registeredc_callables: HashMap::new(),
        }
    }
    pub fn encode(&mut self, obj: CdcValue) -> Vec<u8>{
        let mut buffer: Vec<u8> = Vec::new();
        self.encode_value(&mut buffer, &obj);
        buffer
    }
    fn encode_string(buffer: &mut Vec<u8>, string: &String){
        let str_bytes = string.as_bytes();
        let len = str_bytes.len() as u64;
        buffer.extend(&len.to_le_bytes());
        buffer.extend(str_bytes);
    }

    fn encode_value(&mut self, buffer: &mut Vec<u8>, value: &CdcValue) {
        buffer.push(value.discriminant());
        match value {
            CdcValue::NONE => {
                // No additional data for None
            }
            CdcValue::BOOL(b) => {
                buffer.push(if *b { 1 } else { 0 });
            }
            CdcValue::INTEGER(i) => {
                buffer.extend(&i.to_le_bytes());
            }
            CdcValue::FLOAT(f) => {
                buffer.extend(&f.to_le_bytes());
            }
            CdcValue::STRING(s) => {
                CdcEncoder::encode_string(buffer, s);
            }
            CdcValue::LIST(list) => {
                let len = list.len() as u64;
                buffer.extend(&len.to_le_bytes());
                for item in list {
                    self.encode_value(buffer, item);
                }
            }
            CdcValue::MAP(map) => {
                let len = map.len() as u64;
                buffer.extend(&len.to_le_bytes());
                for (key, value) in map {
                    CdcEncoder::encode_string(buffer, key);
                    self.encode_value(buffer, value);
                }
            }
            CdcValue::SLICE(slice) => {
                // Encode start value
                if let Some(start) = &slice.start {
                    self.encode_value(buffer, &CdcValue::INTEGER(*start));
                } else {
                    self.encode_value(buffer, &CdcValue::NONE);
                }
                // Encode stop value
                if let Some(stop) = &slice.stop {
                    self.encode_value(buffer, &CdcValue::INTEGER(*stop));
                } else {
                    self.encode_value(buffer, &CdcValue::NONE);
                }
            }
            CdcValue::INDEXABLE(indexable) => {
                // Encode item
                self.encode_value(buffer, &CdcValue::ITEM(indexable.item.clone()));
                // Encode token
                CdcEncoder::encode_string(buffer, &indexable.token);
                // Encode size
                buffer.extend(&indexable.size.to_le_bytes());
            }
            CdcValue::VEC3D(v) => {
                buffer.extend(&v.x.to_le_bytes());
                buffer.extend(&v.y.to_le_bytes());
                buffer.extend(&v.z.to_le_bytes());
            }
            CdcValue::VEC2D(v) => {
                buffer.extend(&v.x.to_le_bytes());
                buffer.extend(&v.y.to_le_bytes());
            },
            CdcValue::COMMAND(cmd) => {
                let name_bytes = cmd.name.as_bytes();
                let name_len = name_bytes.len() as u64;
                buffer.extend(&name_len.to_le_bytes());
                buffer.extend(name_bytes);
            },
            CdcValue::BLOB(data) => {
                let len = data.len() as u64;
                buffer.extend(&len.to_le_bytes());
                buffer.extend(data);
            },
            CdcValue::CALLABLE(func) => {
                let raw_pointer = func as *const _ as u64;
                self.registeredc_callables.insert(raw_pointer, *func);
                CdcEncoder::encode_string(buffer, &raw_pointer.to_string());
                CdcEncoder::encode_string(buffer, &String::from("rust function"));
            }
            CdcValue::ERROR(error) => {
                CdcEncoder::encode_string(buffer, &error.id);
                CdcEncoder::encode_string(buffer, &error.text);
                buffer.extend(&error.line.to_le_bytes());
            }  
            CdcValue::ITEM(item) => {
                // Encode Item: id (string), category (i64), stage (i64)
                CdcEncoder::encode_string(buffer, &item.id);
                buffer.extend(&(item.category as i64).to_le_bytes());
                buffer.extend(&(item.stage as i64).to_le_bytes());
            }
            CdcValue::TRAIT(trait_obj) => {
                // Encode Trait: id (string), args (CdcList), kwargs (CdcDict)
                CdcEncoder::encode_string(buffer, &trait_obj.id);
                self.encode_value(buffer, &CdcValue::LIST(trait_obj.args.clone()));
                self.encode_value(buffer, &CdcValue::MAP(trait_obj.kwargs.clone()));
            }
            CdcValue::OBJECT(obj) => {
                // Type ID (string)
                CdcEncoder::encode_string(buffer, &obj.type_id);
                // Repr (string)
                CdcEncoder::encode_string(buffer, &obj.repr);
                // Attributes count (i64)
                let attr_count = obj.attributes.len() as i64;
                buffer.extend(&attr_count.to_le_bytes());
                // Encode each attribute
                for (key, value) in &obj.attributes {
                    CdcEncoder::encode_string(buffer, key);
                    self.encode_value(buffer, value);
                }
            }
            CdcValue::ARRAY(arr) => {
                // Encode project
                self.encode_value(buffer, &arr.project);
                // Encode item
                self.encode_value(buffer, &arr.item);
                // Encode key
                CdcEncoder::encode_string(buffer, &arr.key);
                // Encode index path
                let index_len = arr.index.len() as i64;
                buffer.extend(&index_len.to_le_bytes());
                for idx in &arr.index {
                    buffer.extend(&idx.to_le_bytes());
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
            CdcValue::PACKAGE(pkg) => {
                CdcEncoder::encode_string(buffer, &pkg.reference);
                let metadata_count = pkg.metadata.len() as i64;
                buffer.extend(&metadata_count.to_le_bytes());
                for (key, value) in &pkg.metadata {
                    CdcEncoder::encode_string(buffer, key);
                    self.encode_value(buffer, value);
                }
            }
            CdcValue::RESOURCE_ACCESS => {
                // No additional data for ResourceAccess
            }
        }
    }

    fn decode_int(&self, buffer: &mut &[u8]) -> Result<i64, DecodeError> {
        if buffer.len() < 8 {
            return Err(DecodeError::MissingData);
        }
        let mut int_bytes = [0u8; 8];
        int_bytes.copy_from_slice(&buffer[..8]);
        *buffer = &buffer[8..];
        Ok(i64::from_le_bytes(int_bytes))
    }
    fn decode_string(&self, buffer: &mut &[u8]) -> Result<String, DecodeError> {
        let len = self.decode_int(buffer)? as usize;
        if buffer.len() < len {
            return Err(DecodeError::MissingData);
        }
        let s = String::from_utf8_lossy(&buffer[..len]).to_string();
        *buffer = &buffer[len..];
        Ok(s)
    }
    pub fn decode_value(&self, buffer: &mut &[u8]) -> Result<CdcValue, DecodeError> {
        if buffer.is_empty() {
            return Err(DecodeError::MissingData);
        }
        let type_byte = buffer[0];
        *buffer = &buffer[1..];
        match type_byte {
            x if x == CdcType::NONE as u8 => Ok(CdcValue::NONE),
            x if x == CdcType::BOOLEAN as u8 => {
                if buffer.is_empty() {
                    return Err(DecodeError::MissingData);
                }
                let b = buffer[0] != 0;
                *buffer = &buffer[1..];
                Ok(CdcValue::BOOL(b))
            }
            x if x == CdcType::INTEGER as u8 => {
                Ok(CdcValue::INTEGER(self.decode_int(buffer)?))
            }
            x if x == CdcType::FLOAT as u8 => {
                if buffer.len() < 8 {
                    return Err(DecodeError::MissingData);
                }
                let mut float_bytes = [0u8; 8];
                float_bytes.copy_from_slice(&buffer[..8]);
                *buffer = &buffer[8..];
                Ok(CdcValue::FLOAT(f64::from_le_bytes(float_bytes)))
            }
            x if x == CdcType::STRING as u8 => {
                Ok(CdcValue::STRING(self.decode_string(buffer)?))
            }
            x if x == CdcType::LIST as u8 => {
                let len = self.decode_int(buffer)? as usize;
                let mut result_list: Vec<CdcValue> = Vec::with_capacity(len);
                for _ in 0..len{
                    result_list.push(self.decode_value(buffer)?);
                }
                Ok(CdcValue::LIST(result_list))
                    
            }
            x if x == CdcType::MAP as u8 => {
                let len = self.decode_int(buffer)? as usize;
                let mut result_map: CdcDict = HashMap::with_capacity(len);
                for _ in 0..len{
                    result_map.insert(self.decode_string(buffer)?, self.decode_value(buffer)?);
                }
                Ok(CdcValue::MAP(result_map))
                    
            }
            x if x == CdcType::SLICE as u8 => {
                let start = self.decode_value(buffer)?;
                let stop = self.decode_value(buffer)?;
                
                let start_opt = if let CdcValue::NONE = start {
                    None
                } else if let CdcValue::INTEGER(val) = start {
                    Some(val)
                } else {
                    return Err(DecodeError::UnknownType);
                };
                
                let stop_opt = if let CdcValue::NONE = stop {
                    None
                } else if let CdcValue::INTEGER(val) = stop {
                    Some(val)
                } else {
                    return Err(DecodeError::UnknownType);
                };
                
                Ok(CdcValue::SLICE(Slice {
                    start: start_opt,
                    stop: stop_opt,
                }))
            }
            x if x == CdcType::INDEXABLE as u8 => {
                let item_value = self.decode_value(buffer)?;
                let token = self.decode_string(buffer)?;
                let size = self.decode_int(buffer)?;
                
                // Extract Item from the decoded value
                let item = match item_value {
                    CdcValue::ITEM(item) => item,
                    _ => return Err(DecodeError::UnknownType),
                };
                
                Ok(CdcValue::INDEXABLE(Indexable {
                    item,
                    token,
                    size,
                }))
            }
            x if x == CdcType::VEC3D as u8 => {
                if buffer.len() < 24 {
                    return Err(DecodeError::MissingData);
                }
                let mut x_bytes = [0u8; 8];
                let mut y_bytes = [0u8; 8];
                let mut z_bytes = [0u8; 8];
                x_bytes.copy_from_slice(&buffer[..8]);
                y_bytes.copy_from_slice(&buffer[8..16]);
                z_bytes.copy_from_slice(&buffer[16..24]);
                *buffer = &buffer[24..];
                Ok(CdcValue::VEC3D(Vec3d {
                    x: f64::from_le_bytes(x_bytes),
                    y: f64::from_le_bytes(y_bytes),
                    z: f64::from_le_bytes(z_bytes),
                }))
            }
            x if x == CdcType::VEC2D as u8 => {
                if buffer.len() < 16 {
                    return Err(DecodeError::MissingData);
                }
                let mut x_bytes = [0u8; 8];
                let mut y_bytes = [0u8; 8];
                x_bytes.copy_from_slice(&buffer[..8]);
                y_bytes.copy_from_slice(&buffer[8..16]);
                *buffer = &buffer[16..];
                Ok(CdcValue::VEC2D(Vec2d {
                    x: f64::from_le_bytes(x_bytes),
                    y: f64::from_le_bytes(y_bytes),
                }))
            }
            x if x == CdcType::COMMAND as u8 => {
                let name = self.decode_string(buffer)?;
                Ok(CdcValue::COMMAND(Command { name }))
            }
            x if x == CdcType::BLOB as u8 => {
                let len = self.decode_int(buffer)? as usize;
                if buffer.len() < len {
                    return Err(DecodeError::MissingData);
                }
                let data = buffer[..len].to_vec();
                *buffer = &buffer[len..];
                Ok(CdcValue::BLOB(data))
            }
            x if x == CdcType::CALLABLE as u8 => {
                let pointer_str = self.decode_string(buffer)?;
                let pointer = pointer_str.parse::<u64>().map_err(|_| DecodeError::UnknownType)?;
                if let Some(func) = self.registeredc_callables.get(&pointer) {
                    Ok(CdcValue::CALLABLE(*func))
                } else {
                    Err(DecodeError::MissingFunction)
                }
            }
            x if x == CdcType::ERROR as u8 => {
                let id = self.decode_string(buffer)?;
                let text = self.decode_string(buffer)?;
                let line = self.decode_int(buffer)?;
                Ok(CdcValue::ERROR(CdcError { id, text, line }))
            }
            x if x == CdcType::TRAIT as u8 => {
                // Decode Trait: id (string), args (CdcList), kwargs (CdcDict)
                let id = self.decode_string(buffer)?;
                let args_value = self.decode_value(buffer)?;
                let kwargs_value = self.decode_value(buffer)?;
                
                // Extract LIST and MAP from decoded values
                let args = match args_value {
                    CdcValue::LIST(list) => list,
                    _ => return Err(DecodeError::UnknownType),
                };
                
                let kwargs = match kwargs_value {
                    CdcValue::MAP(map) => map,
                    _ => return Err(DecodeError::UnknownType),
                };
                
                Ok(CdcValue::TRAIT(Trait { id, args, kwargs }))
            }
            x if x == CdcType::ITEM as u8 => {
                // Decode Item: id (string), category (i64), stage (i64)
                let id = self.decode_string(buffer)?;
                let category = self.decode_int(buffer)? as i32;
                let stage = self.decode_int(buffer)? as i32;
                Ok(CdcValue::ITEM(Item { id, category, stage }))
            }
            x if x == CdcType::RESOURCE_ACCESS as u8 => {
                // ResourceAccess has no additional data
                Ok(CdcValue::RESOURCE_ACCESS)
            }
            x if x == CdcType::OBJECT as u8 => {
                let type_id = self.decode_string(buffer)?;
                let repr = self.decode_string(buffer)?;
                let attr_count = self.decode_int(buffer)? as usize;
                
                let mut attributes = HashMap::new();
                for _ in 0..attr_count {
                    let key = self.decode_string(buffer)?;
                    let value = self.decode_value(buffer)?;
                    attributes.insert(key, value);
                }
                
                Ok(CdcValue::OBJECT(Object { type_id, repr, attributes }))
            }
            x if x == CdcType::ARRAY as u8 => {
                let project = self.decode_value(buffer)?;
                let item = self.decode_value(buffer)?;
                let key = self.decode_string(buffer)?;
                
                let index_len = self.decode_int(buffer)? as usize;
                let mut index = Vec::new();
                for _ in 0..index_len {
                    index.push(self.decode_int(buffer)?);
                }
                
                if buffer.is_empty() {
                    return Err(DecodeError::MissingData);
                }
                let selected = buffer[0] != 0;
                *buffer = &buffer[1..];
                
                if buffer.is_empty() {
                    return Err(DecodeError::MissingData);
                }
                let transformation = if buffer[0] != 0 {
                    *buffer = &buffer[1..];
                    Some(Box::new(self.decode_value(buffer)?))
                } else {
                    *buffer = &buffer[1..];
                    None
                };
                
                Ok(CdcValue::ARRAY(Array { project: Box::new(project), item: Box::new(item), key, index, selected, transformation }))
            }
            x if x == CdcType::PACKAGE as u8 => {
                let reference = self.decode_string(buffer)?;
                let metadata_count = self.decode_int(buffer)? as usize;
                
                let mut metadata = HashMap::new();
                for _ in 0..metadata_count {
                    let key = self.decode_string(buffer)?;
                    let value = self.decode_value(buffer)?;
                    metadata.insert(key, value);
                }
                
                Ok(CdcValue::PACKAGE(Package { reference, metadata }))
            }
            _ => Err(DecodeError::UnknownType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn load_expected(name: &str) -> Vec<u8> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let path = root.join("test_binaries").join(format!("{}_expected.bin", name));
        fs::read(path).expect(&format!("Failed to read {}_expected.bin", name))
    }

    #[test]
    fn test_none_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::NONE;
        let encoded = encoder.encode(value);
        let expected = load_expected("none");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_bool_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::BOOL(true);
        let encoded = encoder.encode(value);
        let expected = load_expected("bool");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_integer_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::INTEGER(42);
        let encoded = encoder.encode(value);
        let expected = load_expected("integer");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_float_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::FLOAT(3.14);
        let encoded = encoder.encode(value);
        let expected = load_expected("float");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_string_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::STRING("hello world".to_string());
        let encoded = encoder.encode(value);
        let expected = load_expected("string");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_list_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::LIST(vec![
            CdcValue::INTEGER(1),
            CdcValue::INTEGER(2),
            CdcValue::INTEGER(3),
            CdcValue::STRING("test".to_string()),
        ]);
        let encoded = encoder.encode(value);
        let expected = load_expected("list");
        assert_eq!(encoded, expected);
    }
/* This test can't work as the order in a HashMap is not deterministic, so the encoded bytes can differ between runs.
    #[test]
    fn test_map_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let mut map = CdcDict::new();
        map.insert("key1".to_string(), CdcValue::STRING("value1".to_string()));
        map.insert("key2".to_string(), CdcValue::INTEGER(42));
        map.insert("key3".to_string(), CdcValue::LIST(vec![CdcValue::INTEGER(1), CdcValue::INTEGER(2)]));
        for (key, value) in &map {
            println!("Map entry: {} => {:?}", key, value);
        }
        let value = CdcValue::MAP(map);
        let encoded = encoder.encode(value);
        let expected = load_expected("map");
        assert_eq!(encoded, expected);
    }
 */
    #[test]
    fn test_slice_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let slice = Slice {
            start: Some(1),
            stop: Some(10),
        };
        let value = CdcValue::SLICE(slice);
        let encoded = encoder.encode(value);
        let expected = load_expected("slice");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_slice_encoding_roundtrip() {
        let mut encoder = CdcEncoder::new();
        let original_slice = Slice {
            start: Some(2),
            stop: Some(20),
        };
        let value = CdcValue::SLICE(original_slice.clone());
        let encoded = encoder.encode(value);
        
        // Decode the encoded value
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        
        if let CdcValue::SLICE(decoded_slice) = decoded {
            assert_eq!(decoded_slice.start, original_slice.start);
            assert_eq!(decoded_slice.stop, original_slice.stop);
        } else {
            panic!("Expected SLICE, found {:?}", decoded);
        }
    }

    #[test]
    fn test_indexable_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let indexable = Indexable {
            item: Item {
                id: "item123".to_string(),
                category: 42,
                stage: 7,
            },
            token: "test_token".to_string(),
            size: 100,
        };
        let value = CdcValue::INDEXABLE(indexable);
        let encoded = encoder.encode(value);
        let expected = load_expected("indexable");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_indexable_encoding_roundtrip() {
        let mut encoder = CdcEncoder::new();
        let original_indexable = Indexable {
            item: Item {
                id: "test_item".to_string(),
                category: 99,
                stage: 3,
            },
            token: "roundtrip_token".to_string(),
            size: 250,
        };
        let value = CdcValue::INDEXABLE(original_indexable.clone());
        let encoded = encoder.encode(value);
        
        // Decode the encoded value
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        
        if let CdcValue::INDEXABLE(decoded_indexable) = decoded {
            assert_eq!(decoded_indexable.item, original_indexable.item);
            assert_eq!(decoded_indexable.token, original_indexable.token);
            assert_eq!(decoded_indexable.size, original_indexable.size);
        } else {
            panic!("Expected INDEXABLE, found {:?}", decoded);
        }
    }

    #[test]
    fn test_vec2d_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::VEC2D(Vec2d { x: 1.5, y: 2.5 });
        let encoded = encoder.encode(value);
        let expected = load_expected("vec2d");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_vec3d_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::VEC3D(Vec3d { x: 1.1, y: 2.2, z: 3.3 });
        let encoded = encoder.encode(value);
        let expected = load_expected("vec3d");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_command_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::COMMAND(Command { name: "example_command".to_string() });
        let encoded = encoder.encode(value);
        let expected = load_expected("command");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_blob_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::BLOB(b"binary data example".to_vec());
        let encoded = encoder.encode(value);
        let expected = load_expected("blob");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_item_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::ITEM(Item {
            id: "item123".to_string(),
            category: 42,
            stage: 7,
        });
        let encoded = encoder.encode(value.clone());
        let expected = load_expected("item");
        assert_eq!(encoded, expected);

        // Also test decode roundtrip
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_resource_access_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::RESOURCE_ACCESS;
        let encoded = encoder.encode(value);
        let expected = load_expected("resource_access");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_resource_access_encoding_roundtrip() {
        let mut encoder = CdcEncoder::new();
        let value = CdcValue::RESOURCE_ACCESS;
        let encoded = encoder.encode(value.clone());
        
        // Decode the encoded value
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_error_encoding_matches_python() {
        let mut encoder = CdcEncoder::new();
        let error = CdcError {
            id: "error_id_123".to_string(),
            text: "An error occurred".to_string(),
            line: 42,
        };
        let value = CdcValue::ERROR(error);
        let encoded = encoder.encode(value);
        let expected = load_expected("error");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_error_encoding_roundtrip() {
        let mut encoder = CdcEncoder::new();
        let original_error = CdcError {
            id: "test_error_id".to_string(),
            text: "Test error message".to_string(),
            line: 99,
        };
        let value = CdcValue::ERROR(original_error.clone());
        let encoded = encoder.encode(value);
        
        // Decode the encoded value
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        
        if let CdcValue::ERROR(decoded_error) = decoded {
            assert_eq!(decoded_error.id, original_error.id);
            assert_eq!(decoded_error.text, original_error.text);
            assert_eq!(decoded_error.line, original_error.line);
        } else {
            panic!("Expected ERROR, found {:?}", decoded);
        }
    }

    #[test]
    fn test_trait_encoding_roundtrip() {
        let mut encoder = CdcEncoder::new();
        let trait_obj = Trait {
            id: "Tom::Test::SimpleType".to_string(),
            args: vec![
                CdcValue::INTEGER(1),
                CdcValue::INTEGER(2),
                CdcValue::STRING("test".to_string()),
            ],
            kwargs: {
                let mut map = HashMap::new();
                map.insert("key".to_string(), CdcValue::STRING("value".to_string()));
                map.insert("num".to_string(), CdcValue::INTEGER(42));
                map
            },
        };
        let value = CdcValue::TRAIT(trait_obj);
        let encoded = encoder.encode(value.clone());
        
        // Decode the encoded value
        let mut slice = encoded.as_slice();
        let decoded = encoder.decode_value(&mut slice).unwrap();
        
        // Compare the decoded value with the original
        assert_eq!(decoded, value);
    }

}