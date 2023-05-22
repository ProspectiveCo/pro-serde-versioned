use std::borrow::Cow;

use pro_serde_versioned::*;
use serde::*;

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

#[derive(Debug, PartialEq, VersionedUpgrade, VersionedSerialize, VersionedDeserialize, Clone)]
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

    let wrapper: MyStructVersion = MyStructVersion::versioned_deserialize(&value)?;

    assert_eq!(
        wrapper,
        MyStructVersion::V1(MyStructV1 {
            field1: "value1".to_string()
        })
    );

    let serialized_wrapper: serde_json::Value = wrapper.versioned_serialize()?;

    assert_eq!(serialized_wrapper, value);

    Ok(())
}

#[test]
fn test_msgpack_serde() -> Result<(), Box<dyn std::error::Error>> {
    let json_value: serde_json::Value = serde_json::from_str(V1_STRUCT)?;
    let versioned_struct: MyStructVersion = MyStructVersion::versioned_deserialize(&json_value)?;
    let serialized_wrapper: MsgPackBytes = MyStructVersion::versioned_serialize(&versioned_struct)?;

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
    let _: MyStructVersion =
        MyStructVersion::versioned_deserialize(&MsgPackBytes(serialized_wrapper))?;

    Ok(())
}

#[test]
fn test_change_field_representation() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV1 {
        field1: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV2 {
        field1: Vec<u8>,
    }

    impl Upgrade<MyStructV2> for MyStructV1 {
        fn upgrade(self: MyStructV1) -> MyStructV2 {
            MyStructV2 {
                field1: self.field1.into_bytes(),
            }
        }
    }

    #[derive(
        Debug, PartialEq, VersionedUpgrade, VersionedSerialize, VersionedDeserialize, Clone,
    )]
    enum MyStructVersion {
        V1(MyStructV1),
        V2(MyStructV2),
    }

    let v1_struct = MyStructVersion::V1(MyStructV1 {
        field1: "value1".to_string(),
    });

    let serialized_wrapper: serde_json::Value = MyStructVersion::versioned_serialize(&v1_struct)?;

    assert_eq!(
        serialized_wrapper,
        serde_json::json!({
            "version_number": 1,
            "data": {
                "field1": "value1"
            }
        })
    );

    let v2_struct =
        MyStructVersion::versioned_deserialize(&serialized_wrapper)?.upgrade_to_latest();

    assert_eq!(
        v2_struct,
        MyStructV2 {
            field1: "value1".to_string().into_bytes()
        }
    );

    Ok(())
}

#[test]
fn test_remove_field() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV1 {
        field1: String,
        field2: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV2 {
        field1: String,
    }

    impl Upgrade<MyStructV2> for MyStructV1 {
        fn upgrade(self: MyStructV1) -> MyStructV2 {
            MyStructV2 {
                field1: self.field1,
            }
        }
    }

    #[derive(
        Debug, PartialEq, VersionedUpgrade, VersionedSerialize, VersionedDeserialize, Clone,
    )]
    enum MyStructVersion {
        V1(MyStructV1),
        V2(MyStructV2),
    }

    let v1_struct = MyStructVersion::V1(MyStructV1 {
        field1: "value1".to_string(),
        field2: "value2".to_string(),
    });

    let serialized_wrapper: serde_json::Value = MyStructVersion::versioned_serialize(&v1_struct)?;

    assert_eq!(
        serialized_wrapper,
        serde_json::json!({
            "version_number": 1,
            "data": {
                "field1": "value1",
                "field2": "value2"
            }
        })
    );

    let v2_struct =
        MyStructVersion::versioned_deserialize(&serialized_wrapper)?.upgrade_to_latest();

    let v2_serialized: serde_json::Value = MyStructVersion::V2(v2_struct).versioned_serialize()?;

    assert_eq!(
        v2_serialized,
        serde_json::json!({
            "version_number": 2,
            "data": {
                "field1": "value1"
            }
        })
    );

    Ok(())
}

#[test]
fn test_change_enum_values() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV1 {
        field1: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV2 {
        field1: usize,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MyStructV3 {
        field1: bool,
        field2: usize,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    enum MyStructVersion1 {
        V1(MyStructV1),
        V2(MyStructV2),
    }

    let v2_struct = MyStructVersion1::V2(MyStructV2 { field1: 123 });

    let serialized_wrapper = MsgPackBytes::serialize_format(&v2_struct)?;

    // assert_eq!(
    //     serialized_wrapper,
    //     serde_json::json!({
    //         "version_number": 2,
    //         "data": {
    //             "field1": 123
    //         }
    //     })
    // );

    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    enum MyStructVersion2 {
        V2(MyStructV2),
        V3(MyStructV3),
    }

    let v2_struct: MyStructVersion2 = MsgPackBytes::deserialize_format(&serialized_wrapper)?;
    assert_eq!(v2_struct, MyStructVersion2::V2(MyStructV2 { field1: 123 }));
    Ok(())
}
