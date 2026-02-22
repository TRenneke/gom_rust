use crate::{Vec2d, Vec3d, Command, Item};
use std::{collections::HashMap, fmt, hash::Hash, os::raw, result};


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
            CdcValue::COMMAND(_) => CdcType::COMMAND,
            CdcValue::CALLABLE(_) => CdcType::CALLABLE,
            CdcValue::VEC2D(_) => CdcType::VEC2D,
            CdcValue::VEC3D(_) => CdcType::VEC3D,
            CdcValue::BLOB(_) => CdcType::BLOB,
            CdcValue::ITEM(_) => CdcType::ITEM,
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
    CALLABLE(CdcCallable) = 11,
    VEC2D(Vec2d) = 17,
    VEC3D(Vec3d) = 18,
    COMMAND(Command) = 10,
    BLOB(Vec<u8>) = 20,
    ITEM(Item) = 8,
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
    pub fn expect_item(self) -> Item {
        if let CdcValue::ITEM(b) = self {b} else {panic!("Expected ITEM, found {:?}", self);}
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
            CdcValue::ITEM(item) => {
                // Encode Item: id (string), category (i64), stage (i64)
                CdcEncoder::encode_string(buffer, &item.id);
                buffer.extend(&(item.category as i64).to_le_bytes());
                buffer.extend(&(item.stage as i64).to_le_bytes());
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
            x if x == CdcType::ITEM as u8 => {
                // Decode Item: id (string), category (i64), stage (i64)
                let id = self.decode_string(buffer)?;
                let category = self.decode_int(buffer)? as i32;
                let stage = self.decode_int(buffer)? as i32;
                Ok(CdcValue::ITEM(Item { id, category, stage }))
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

}