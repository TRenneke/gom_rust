# Remaining CdcTypes to Implement

This document tracks the remaining CdcTypes that need to be implemented in the Rust GOM API library.

## Completed CdcTypes ✓

- NONE (0) ✓
- BOOLEAN (1) ✓
- INTEGER (2) ✓
- FLOAT (3) ✓
- STRING (4) ✓
- LIST (5) ✓
- MAP (6) ✓
- SLICE (7) ✓
- ITEM (8) ✓
- INDEXABLE (9) ✓
- COMMAND (10) ✓
- CALLABLE (11) ✓
- ERROR (12) ✓
- TRAIT (13) ✓
- VEC2D (17) ✓
- VEC3D (18) ✓
- RESOURCE_ACCESS (19) ✓
- BLOB (20) ✓

## Remaining CdcTypes to Implement (3 total)
### Object  - LOW PRIORITY -> This is never used.
**Structure:**
- `params: CdcValue` - Object parameters (typically a MAP or other CdcValue)

**Python Reference:**
```python
elif obj_type == CdcEncoder.Type.OBJECT:
    return gom.Object.from_params(self.decodeValue(s, context))
```

**Implementation Notes:**
- Represents generic GOM Object instances
- Takes any CdcValue as parameter (usually MAP with properties)
- Requires gom.Object wrapper type

### 3. ARRAY (Type 15) - MEDIUM PRIORITY
**Structure:**
- `project: CdcValue` - Project reference
- `item: Item` - Item reference
- `key: String` - Array property key
- `index: int` - Index value
- `selected: bool` - Selection status
- `transformation: CdcValue` - Transformation matrix/data

**Python Reference:**
```python
elif obj_type == CdcEncoder.Type.ARRAY:
    project = self.decodeValue(s, context)
    item = self.decodeValue(s, context)
    key = self.decodeStr(s)
    index = self.decodeValue(s, context)
    selected = self.decodeBool(s)
    transformation = self.decodeValue(s, context)
    return gom.Array(project=project, item=item, key=key, index=index,
                     selected=selected, transformation=transformation)
```

**Implementation Notes:**
- Complex type combining multiple CdcValues
- Represents array properties/data access patterns
- selected is encoded as single boolean byte

### 4. PACKAGE (Type 16) - LOW PRIORITY
**Structure:**
- `dims: i64` - Number of dimensions
- `shape: Vec<i64>` - Array shape/dimensions
- `data_type: i32` - NumPy dtype enum value
- `use_shared_memory: bool` - Flag for shared memory usage
- `data: Vec<u8>` or `shared_memory_key: String` - Actual array data

**Python Reference:**
```python
self.encodeType(buffer, CdcEncoder.Type.PACKAGE)
self.encodeInt(buffer, len(obj.shape))
for i in obj.shape:
    self.encodeInt(buffer, i)
if obj.dtype == np.int8:
    self.encodeInt(buffer, Encoder.PackageType.INT_8.value)
# ... dtype handling ...
if use_shared_memory:
    self.encodeBool(buffer, True)
    self.encodeStr(buffer, shared_memory_key)
else:
    self.encodeBool(buffer, False)
    buffer.extend(obj.tobytes())
```

**Implementation Notes:**
- Handles NumPy array serialization
- Supports both direct data and shared memory references
- Complex dtype mapping required
- May require separate PackageType enum
- On the rust side store the data in nalgebra arrays rather than rust arrays.

## Implementation Order Recommendation

1. **ARRAY (15)** - Complex, depends on multiple CdcValue types
2. **PACKAGE (16)** - NumPy-specific, lowest priority
3. **OBJECT (14)** - Next priority: Depends on understanding TRAIT


## Testing Strategy

For each CdcType:
1. Add struct definition to `lib.rs`
2. Add variant to `CdcValue` enum
3. Implement encode in `encode_value()`
4. Implement decode in `decode_value()`
5. Update `From<&CdcValue> for CdcType`
6. Generate test binaries using `generate_test_binaries.py`
7. Add Rust tests:
   - `test_{name}_encoding_matches_python()`
   - `test_{name}_encoding_roundtrip()`
8. Run: `cargo test --lib encoding::tests::{test_name}`

## Notes

- All types use little-endian byte ordering
- String encoding: length (i64) + UTF-8 bytes
- Recursive types (TRAIT, OBJECT, ARRAY) encode CdcValue references directly
- Test binaries are generated from Python's CdcEncoder for validation
