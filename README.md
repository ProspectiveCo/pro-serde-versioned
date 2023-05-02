# Versioning

This Rust project provides a simple and convenient way to handle versioning and upgrading of your data structures using Serde for serialization and deserialization.

## Features
* Define multiple versions of your data structure.
* Automatically deserialize different versions of your data structure using Serde.
* Easily upgrade from one version of the structure to another.
* Upgrade to the latest version of the structure in a single function call.

## Example

The project demonstrates upgrading between three versions of a structure named MyStruct. Each version of the structure has a new field added:

```rust
pub struct MyStructV1 {
    field1: String,
}

pub struct MyStructV2 {
    field1: String,
    new_field: String,
}

pub struct MyStructV3 {
    field1: String,
    new_field: String,
    second_new_field: String,
}
```

Each version of the structure is part of an enumeration MyStructVersion, which uses the UpgradableEnum derive macro to help with deserialization and upgrading:

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, UpgradableEnum)]
#[serde(tag = "version")]
pub enum MyStructVersion {
    V1(MyStructV1),
    V2(MyStructV2),
    V3(MyStructV3),
}
```

Implement the Upgrade trait for each pair of adjacent versions, defining the upgrade logic:

```rust
impl Upgrade<MyStructV2> for MyStructV1 {
    fn upgrade(self: MyStructV1) -> MyStructV2 {
        MyStructV2 {
            field1: self.field1.to_uppercase(),
            new_field: "default_value".to_string(),
        }
    }
}

impl Upgrade<MyStructV3> for MyStructV2 {
    fn upgrade(self: MyStructV2) -> MyStructV3 {
        MyStructV3 {
            field1: self.field1,
            new_field: self.new_field,
            second_new_field: "default_value_v3".to_string(),
        }
    }
}
```

The Upgrade trait implementations are used to upgrade between different versions of the structure. The Upgradable trait implementation allows upgrading to the latest version of the structure in a single function call:

```rust
let result = serde_json::from_str::<'static, MyStructVersion>(V1_STRUCT)?;
let upgraded = result.upgrade_to_latest();
```

