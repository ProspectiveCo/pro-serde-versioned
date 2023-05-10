# Versioning

This Rust project provides a simple and convenient way to handle versioning and upgrading of your data structures using Serde for serialization and deserialization.

# Features
- Derivable traits for chaining upgrades from older versions to the latest version of a structure.
- Generic serialization and deserialization for any supported format.
- Versioned data envelopes for partial deserialization and version-aware handling.

# Examples

Here's an example of how to use this library with a simple structure that has two versions:

```rust
use serde::{Deserialize, Serialize};
use versioning::*;
use versioning_derive::{UpgradableEnum, VersionedDeserialize, VersionedSerialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV1 {
    field1: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV2 {
    field1: String,
    new_field: String,
}

#[derive(Debug, PartialEq, UpgradableEnum, VersionedSerialize, VersionedDeserialize, Clone)]
enum MyStructVersion {
    V1(MyStructV1),
    V2(MyStructV2),
}

impl Upgrade<MyStructV2> for MyStructV1 {
    fn upgrade(self: MyStructV1) -> MyStructV2 {
        MyStructV2 {
            field1: self.field1.to_uppercase(),
            new_field: "default_value".to_string(),
        }
    }
}

// Serializing MyStructV1 to JSON format
let v1 = MyStructV1 {
    field1: "value1".to_string(),
};
let versioned_v1 = MyStructVersion::V1(v1);
let serialized_v1: serde_json::Value = MyStructVersion::serialize(&versioned_v1).unwrap();

// Deserialize MyStructVersion from JSON format
let deserialized_versioned: MyStructVersion =
    MyStructVersion::deserialize(&serialized_v1).unwrap();

// Upgrade MyStructV1 to MyStructV2
let upgraded_v2 = deserialized_versioned.upgrade_to_latest();

assert_eq!(
    upgraded_v2,
    MyStructV2 {
        field1: "VALUE1".to_string(),
        new_field: "default_value".to_string(),
    }
);
```