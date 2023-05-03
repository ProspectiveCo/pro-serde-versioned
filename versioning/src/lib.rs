use std::borrow::Cow;

use serde::{Deserialize, Serialize};

pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

pub trait VersionedStructure<'a>: Serialize + Deserialize<'a> {
    type Deserializer;
    type Serializer;
    fn get_deserializer(data: &'a [u8]) -> Self::Deserializer;
    fn get_serializer() -> Self::Serializer;
}

pub(crate) trait InternalVersionedStructure<'a>: Serialize + Deserialize<'a> {
}

pub trait VersionedSerde<'a>: Sized {
    fn from_versioned_envelope(
        envelope: VersionedEnvelope<'a>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn to_versioned_envelope(&self)
        -> Result<VersionedEnvelope<'a>, Box<dyn std::error::Error>>;
}

pub trait UpgradableEnum {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VersionedEnvelope<'a> {
    version_number: usize,
    data: Cow<'a, [u8]>,
}

impl<'a> VersionedEnvelope<'a> {
    pub fn get_version(&self) -> usize {
        self.version_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rmp_serde::decode::ReadRefReader;
    use serde_json::de::SliceRead;
    use versioning_derive::UpgradableEnum;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct MyStructV1 {
        field1: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct MyStructV2 {
        field1: String,
        new_field: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct MyStructV3 {
        field1: String,
        new_field: String,
        second_new_field: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, UpgradableEnum)]
    #[serde(tag = "version")]
    pub enum MyStructVersion {
        V1(MyStructV1),
        V2(MyStructV2),
        V3(MyStructV3),
    }

    // TODO: Test multiple enums that break out v1, 2, 3, 4, 5, 6 into v1, 2, 3 and v4, 5, 6.
    // TODO: Do partial deserialization
    // TODO: Test fields that change shape (e.g. string to array, etc.)
    // TODO: Test renaming fields

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

    impl<'a> VersionedStructure<'a> for MyStructV1 {
        type Deserializer = serde_json::Deserializer<SliceRead<'a>>;
        type Serializer = serde_json::Serializer<Vec<u8>>;

        fn get_deserializer(data: &'a [u8]) -> Self::Deserializer {
            serde_json::Deserializer::new(serde_json::de::SliceRead::new(data))
        }

        fn get_serializer() -> Self::Serializer {
            serde_json::Serializer::new(Vec::new())
        }
    }

    impl<'a> VersionedStructure<'a> for MyStructV2 {
        type Deserializer = serde_json::Deserializer<SliceRead<'a>>;
        type Serializer = serde_json::Serializer<Vec<u8>>;

        fn get_deserializer(data: &'a [u8]) -> Self::Deserializer {
            serde_json::Deserializer::new(serde_json::de::SliceRead::new(data))
        }

        fn get_serializer() -> Self::Serializer {
            serde_json::Serializer::new(Vec::new())
        }
    }

    impl<'a> VersionedStructure<'a> for MyStructV3 {
        type Deserializer = rmp_serde::Deserializer<ReadRefReader<'a, [u8]>>;
        type Serializer = rmp_serde::Serializer<Vec<u8>>;

        fn get_deserializer(data: &'a [u8]) -> Self::Deserializer {
            rmp_serde::Deserializer::from_read_ref(data)
        }

        fn get_serializer() -> Self::Serializer {
            rmp_serde::Serializer::new(Vec::new())
        }
    }

    impl<'a> VersionedSerde<'a> for MyStructVersion {
        fn from_versioned_envelope(
            envelope: VersionedEnvelope<'a>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            match envelope.version_number {
                1 => {
                    let mut struct_v1_de = MyStructV1::get_deserializer(&envelope.data);
                    Ok(MyStructV1::deserialize(&mut struct_v1_de).map(MyStructVersion::V1)?)
                }
                2 => {
                    let mut struct_v2_de = MyStructV2::get_deserializer(&envelope.data);
                    Ok(MyStructV2::deserialize(&mut struct_v2_de).map(MyStructVersion::V2)?)
                }
                3 => {
                    let mut struct_v3_de = MyStructV3::get_deserializer(&envelope.data);
                    Ok(MyStructV3::deserialize(&mut struct_v3_de).map(MyStructVersion::V3)?)
                }
                _ => Err("Unknown version".into()),
            }
        }

        fn to_versioned_envelope(
            &self,
        ) -> Result<VersionedEnvelope<'a>, Box<dyn std::error::Error>> {
            match self {
                MyStructVersion::V1(struct_v1) => {
                    let mut struct_v1_ser = MyStructV1::get_serializer();
                    struct_v1.serialize(&mut struct_v1_ser)?;
                    Ok(VersionedEnvelope {
                        version_number: 1,
                        data: struct_v1_ser.into_inner().into(),
                    })
                }
                MyStructVersion::V2(struct_v2) => {
                    let mut struct_v2_ser = MyStructV2::get_serializer();
                    struct_v2.serialize(&mut struct_v2_ser)?;
                    Ok(VersionedEnvelope {
                        version_number: 2,
                        data: struct_v2_ser.into_inner().into(),
                    })
                }
                MyStructVersion::V3(struct_v3) => {
                    let mut struct_v3_ser = MyStructV3::get_serializer();
                    struct_v3.serialize(&mut struct_v3_ser)?;
                    Ok(VersionedEnvelope {
                        version_number: 3,
                        data: struct_v3_ser.into_inner().into(),
                    })
                }
            }
        }
    }

    const V1_STRUCT: &'static str = r#"
        {"version": "V1", "field1": "value1"}
    "#;

    #[test]
    fn should_deserialize_versioned_envelope() -> Result<(), Box<dyn std::error::Error>> {
        let envelope = VersionedEnvelope {
            version_number: 1,
            data: V1_STRUCT.as_bytes().into(),
        };
        let result = MyStructVersion::from_versioned_envelope(envelope)?;
        assert_eq!(
            result,
            MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            })
        );
        let upgraded = result.upgrade_to_latest();
        assert_eq!(
            upgraded,
            MyStructV3 {
                field1: "VALUE1".to_string(),
                new_field: "default_value".to_string(),
                second_new_field: "default_value_v3".to_string(),
            }
        );
        let message_pack = MyStructVersion::V3(upgraded.clone()).to_versioned_envelope()?;
        assert_eq!(message_pack.version_number, 3);
        assert_eq!(
            &message_pack.data,
            &rmp_serde::to_vec(&upgraded)?
        );
        Ok(())
    }

    #[test]
    fn should_deserialize_valid_json() -> Result<(), serde_json::Error> {
        let result = serde_json::from_str::<'static, MyStructVersion>(V1_STRUCT)?;
        assert_eq!(
            result,
            MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            })
        );
        Ok(())
    }

    #[test]
    fn should_upgrade_v1_to_v2() -> Result<(), serde_json::Error> {
        let result = serde_json::from_str::<'static, MyStructV1>(V1_STRUCT)?;
        assert_eq!(
            result.upgrade(),
            MyStructV2 {
                field1: "VALUE1".to_string(),
                new_field: "default_value".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn upgrade_to_latest_should_compose_upgrade_fns() -> Result<(), serde_json::Error> {
        let result = serde_json::from_str::<'static, MyStructVersion>(V1_STRUCT)?;
        let upgraded = result.upgrade_to_latest();
        assert_eq!(
            upgraded,
            MyStructV3 {
                field1: "VALUE1".to_string(),
                new_field: "default_value".to_string(),
                second_new_field: "default_value_v3".to_string()
            }
        );
        Ok(())
    }
}
