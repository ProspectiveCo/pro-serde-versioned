// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │  ██████╗ ██████╗  ██████╗   Copyright (C) The Prospective Company         │
// │  ██╔══██╗██╔══██╗██╔═══██╗  All Rights Reserved - April 2022              │
// │  ██████╔╝██████╔╝██║   ██║                                                │
// │  ██╔═══╝ ██╔══██╗██║   ██║  Proprietary and confidential. Unauthorized    │
// │  ██║     ██║  ██║╚██████╔╝  copying of this file, via any medium is       │
// │  ╚═╝     ╚═╝  ╚═╝ ╚═════╝   strictly prohibited.                          │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘
#![doc = include_str!("../../README.md")]

use serde::{Deserialize, Serialize};

/// Derivable trait used to chain upgrade a versioned wrapper to the latest version of a structure (e.g. v1 -> v2 -> ... -> latest)
pub trait UpgradableEnum {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}

/// Defines the next version of a given upgradable type (e.g. mystructv1 -> mystructv1)
pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

/// Allows for serializing to any supported format.
pub trait VersionedSerialize {
    fn serialize<F>(&self) -> Result<F, Box<dyn std::error::Error>>
    where
        F: SerializeFormat;
}

/// Allows for serializing from any supported format.
pub trait VersionedDeserialize: Sized + Clone {
    fn deserialize<'a, F>(data: &'a F) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: DeserializeFormat + Deserialize<'a>;
}

/// Serialize to the underlying format of a given serialization standard. (e.g. [serde_json::Value] for JSON, [std::borrow::Cow] of bytes for MsgPack, etc.)
pub trait SerializeFormat: Sized + Serialize {
    fn versioned_serialize<T>(data: T) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: Serialize;
}

/// Deserialize from the underlying format of a given serialization standard. (e.g. [serde_json::Value] for JSON, [std::borrow::Cow] of bytes for MsgPack, etc.)
pub trait DeserializeFormat: Sized {
    fn versioned_deserialize<'a, T>(&'a self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Deserialize<'a>;
}

/// Versioned wrapper for the underlying data format.
/// Allows for partial deserialization of the data, and for
/// the version number to be used to determine which
/// deserialization method to use.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VersionedEnvelope<T> {
    pub version_number: usize,
    pub data: T,
}

impl SerializeFormat for serde_json::Value {
    fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::to_value(&data)?)
    }
}

impl DeserializeFormat for serde_json::Value {
    fn versioned_deserialize<'a, T>(&'a self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Deserialize<'a>,
    {
        Ok(T::deserialize(self.clone())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::borrow::Cow;

    use versioning_derive::{UpgradableEnum, VersionedDeserialize, VersionedSerialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MsgPackBytes<'a>(
        #[serde(with = "serde_bytes")]
        #[serde(borrow)]
        Cow<'a, [u8]>,
    );

    impl SerializeFormat for MsgPackBytes<'_> {
        fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(MsgPackBytes(Cow::Owned(rmp_serde::to_vec(&data)?)))
        }
    }

    impl<'a> DeserializeFormat for MsgPackBytes<'a> {
        fn versioned_deserialize<'b, T: Deserialize<'b>>(
            &'b self,
        ) -> Result<T, Box<dyn std::error::Error>> {
            match &self.0 {
                Cow::Borrowed(bytes) => Ok(rmp_serde::from_slice(bytes)?),
                Cow::Owned(bytes) => Ok(rmp_serde::from_slice(&bytes)?),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV1 {
        field1: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV2 {
        field1: String,
        new_field: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV3 {
        field1: String,
        new_field: String,
        second_new_field: String,
    }

    #[derive(Debug, PartialEq, UpgradableEnum, VersionedSerialize, VersionedDeserialize, Clone)]
    enum MyStructVersion {
        V1(MyStructV1),
        V2(MyStructV2),
        V3(MyStructV3),
    }

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

    const V1_STRUCT: &'static str = r#"
        {"version_number": 1, "data": {"field1": "value1"}}
    "#;

    #[test]
    fn test_json_serde() -> Result<(), Box<dyn std::error::Error>> {
        let value: serde_json::Value = serde_json::from_str(V1_STRUCT)?;

        // let foo = value.versioned_deserialize()?;
        // let bar = MyStructVersion::from_versioned_envelope(foo)?;

        let wrapper: MyStructVersion = MyStructVersion::deserialize(&value)?;

        assert_eq!(
            wrapper,
            MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            })
        );

        let serialized_wrapper: serde_json::Value = MyStructVersion::serialize(&wrapper)?;

        assert_eq!(serialized_wrapper, value);

        Ok(())
    }

    #[test]
    fn test_msgpack_serde() -> Result<(), Box<dyn std::error::Error>> {
        let json_value: serde_json::Value = serde_json::from_str(V1_STRUCT)?;
        let versioned_struct: MyStructVersion = MyStructVersion::deserialize(&json_value)?;
        let serialized_wrapper: MsgPackBytes = MyStructVersion::serialize(&versioned_struct)?;

        // We want this to be a borrow so that the msgpack deserialize is zero copy
        let serialized_wrapper = serialized_wrapper.0.as_ref().into();

        assert!(match serialized_wrapper {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false,
        });

        let hex = serialized_wrapper
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");

        assert_eq!(hex, "92 01 c4 08 91 a6 76 61 6c 75 65 31");

        // Asserting that serializer is symmetric
        let _: MyStructVersion = MyStructVersion::deserialize(&MsgPackBytes(serialized_wrapper))?;

        Ok(())
    }
}
