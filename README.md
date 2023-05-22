# `pro-serde-versioned`

This crate provides a simple method for versioning and upgrading data structures
when serialized via [`serde`].

# Features
- The `VersionedSerialize` and `VersionedDeserialize` traits allow deriving 
  stable serialization methods for an enum, which will still work if new enum 
  cases are added in the future
- The `VersionedUpgrade` trait defines a enum sequence of struct generations by
  providing a method to upgrade any struct in the sequence to the latest.

# `VersionedSerialize`/`VersionedDeserialize` Examples

```rust
use serde::{Deserialize, Serialize};
use pro_serde_versioned::{
    VersionedSerialize,
    VersionedDeserialize,
};

// Let's say you have two generations of some serialized data structure ...
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV1 {
    field: String
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV2 {
    field1: u32,
    new_field: String,
}

// Derive [`VersionedSerialize`] and [`VersionedDeserialize`]
#[derive(
    VersionedSerialize,
    VersionedDeserialize,
    Debug,
    PartialEq,
    Clone
)]
enum MyStructVersioned {
    V1(MyStructV1),
    V2(MyStructV2),
}

let versioned_struct: MyStructVersioned = 
    MyStructV1 { field: "123".to_string() }.into();

// Serializing `MyStructV1` to `serde_json::Value` format
let serialized_v1: serde_json::Value = versioned_struct.versioned_serialize()?;

// Deserialize `MyStructVersion` from JSON format 
let deserialized_v1 = MyStructVersioned::versioned_deserialize(&serialized_v1)?;
assert_eq!(deserialized_v1, versioned_struct);

# Ok::<(), Box<dyn std::error::Error>>(())
```

# `VersionedUpgrade` Examples

```rust
use pro_serde_versioned::{
    VersionedUpgrade,
    Upgrade,
};

// Given the same two generations of a serialized data structure ...
#[derive(Debug, PartialEq, Clone)]
struct MyStructV1(String);

#[derive(Debug, PartialEq, Clone)]
struct MyStructV2 {
    field1: u32,
    new_field: String,
}

// ... and an impl for the `Upgrade` trait which links them together ...
impl Upgrade<MyStructV2> for MyStructV1 {
    fn upgrade(self: MyStructV1) -> MyStructV2 {
        MyStructV2 {
            field1: self.0.parse().unwrap_or_default(),
            new_field: "default_value".to_string(),
        }
    }
}

// Derive the [`VersionedUpgrade`] trait on a wrapper enum
#[derive(
    VersionedUpgrade,
    Debug,
    PartialEq,
    Clone
)]
enum MyStructVersion {
    V1(MyStructV1),
    V2(MyStructV2),
}

// Now any struct can be upgraded to the latest enum of [`MyStructVersioned`]!
// Upgrade `MyStructV1` to `MyStructV2`.
let upgraded_v2 = 
    MyStructVersion::V1(MyStructV1("123".to_string())).upgrade_to_latest();

assert_eq!(
    upgraded_v2,
    MyStructV2 {
        field1: 123,
        new_field: "default_value".to_string(),
    }
);

# Ok::<(), Box<dyn std::error::Error>>(())
```