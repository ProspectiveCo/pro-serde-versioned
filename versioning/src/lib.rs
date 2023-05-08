use std::borrow::Cow;

use serde::{Deserialize, Serialize};

trait UpgradableEnum {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}

pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VersionNumber(usize);

impl Default for VersionNumber {
    fn default() -> Self {
        Self(1)
    }
}

impl From<usize> for VersionNumber {
    fn from(version_number: usize) -> Self {
        Self(version_number)
    }
}

impl Into<usize> for VersionNumber {
    fn into(self) -> usize {
        self.0
    }
}

pub trait VersionedWrapper<'a>: Sized + Clone {
    type MsgEnvelope: Envelope<'a>;
    fn from_versioned_envelope(
        envelope: Self::MsgEnvelope,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn to_versioned_envelope(&self) -> Result<Self::MsgEnvelope, Box<dyn std::error::Error>>;

    fn deserialize(
        data: <<Self as VersionedWrapper<'a>>::MsgEnvelope as Envelope<'a>>::Data,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let envelope = Self::MsgEnvelope::deserialize(data)?;
        Self::from_versioned_envelope(envelope)
    }

    fn serialize(
        &self,
    ) -> Result<
        <<Self as VersionedWrapper<'a>>::MsgEnvelope as Envelope<'a>>::Data,
        Box<dyn std::error::Error>,
    > {
        self.to_versioned_envelope()?.serialize()
    }
}

pub trait Envelope<'a>: Sized + Clone {
    type Data;
    fn version_number(&'a self) -> VersionNumber;
    fn data(&'a self) -> Self::Data;
    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>>;
    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MsgPackEnvelope<'a> {
    #[serde(default)]
    version_number: VersionNumber,
    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    data: Cow<'a, [u8]>,
}

impl<'a> Envelope<'a> for MsgPackEnvelope<'a> {
    type Data = Cow<'a, [u8]>;

    fn version_number(&'a self) -> VersionNumber {
        self.version_number
    }

    fn data(&'a self) -> Self::Data {
        self.data.clone()
    }

    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>> {
        match data {
            Cow::Borrowed(b) => Ok(rmp_serde::from_slice(b)?),
            Cow::Owned(o) => {
                let borrowed_envelope: MsgPackEnvelope<'_> = rmp_serde::from_slice(&o)?;
                let owned: MsgPackEnvelope<'a> = MsgPackEnvelope {
                    version_number: borrowed_envelope.version_number,
                    data: match borrowed_envelope.data {
                        Cow::Borrowed(b) => Cow::Owned(b.to_vec()),
                        Cow::Owned(o) => Cow::Owned(o),
                    },
                };
                Ok(owned)
            }
        }
    }

    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>> {
        Ok(Cow::Owned(rmp_serde::to_vec(self)?))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct JsonEnvelope<'a> {
    #[serde(default)]
    version_number: VersionNumber,
    #[serde(flatten)]
    data: Cow<'a, serde_json::Value>,
}

impl<'a> Envelope<'a> for JsonEnvelope<'a> {
    type Data = Cow<'a, serde_json::Value>;

    fn version_number(&'a self) -> VersionNumber {
        self.version_number
    }

    fn data(&'a self) -> Self::Data {
        self.data.clone()
    }

    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>> {
        serde_json::from_value(data.into_owned()).map_err(Into::into)
    }

    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>> {
        Ok(Cow::Owned(serde_json::to_value(self)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[derive(Debug, PartialEq, UpgradableEnum, Clone)]
    pub enum MyStructVersion {
        V1(MyStructV1),
        V2(MyStructV2),
        V3(MyStructV3),
    }

    impl<'a> VersionedWrapper<'a> for MyStructVersion {
        type MsgEnvelope = MsgPackEnvelope<'a>;

        fn from_versioned_envelope(
            envelope: Self::MsgEnvelope,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            match envelope.version_number().0 {
                1 => Ok(MyStructVersion::V1(rmp_serde::from_slice(
                    &envelope.data(),
                )?)),
                2 => Ok(MyStructVersion::V2(rmp_serde::from_slice(
                    &envelope.data(),
                )?)),
                3 => Ok(MyStructVersion::V3(rmp_serde::from_slice(
                    &envelope.data(),
                )?)),
                _ => Err("Unknown version".into()),
            }
        }

        fn to_versioned_envelope(&self) -> Result<Self::MsgEnvelope, Box<dyn std::error::Error>> {
            match self {
                MyStructVersion::V1(value) => {
                    let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                    value.serialize(&mut struct_ser)?;
                    Ok(MsgPackEnvelope {
                        version_number: 1.into(),
                        data: Cow::Owned(struct_ser.into_inner().to_owned()),
                    })
                }
                MyStructVersion::V2(value) => {
                    let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                    value.serialize(&mut struct_ser)?;
                    Ok(MsgPackEnvelope {
                        version_number: 2.into(),
                        data: Cow::Owned(struct_ser.into_inner().to_owned()),
                    })
                }
                MyStructVersion::V3(value) => {
                    let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                    value.serialize(&mut struct_ser)?;
                    Ok(MsgPackEnvelope {
                        version_number: 3.into(),
                        data: Cow::Owned(struct_ser.into_inner().to_owned()),
                    })
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct JsonWrapper(MyStructVersion);

    impl<'a> VersionedWrapper<'a> for JsonWrapper {
        type MsgEnvelope = JsonEnvelope<'a>;

        fn from_versioned_envelope(
            envelope: Self::MsgEnvelope,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            match envelope.version_number().0 {
                1 => Ok(JsonWrapper(MyStructVersion::V1(serde_json::from_slice(
                    &envelope.data().to_string().as_bytes(),
                )?))),
                2 => Ok(JsonWrapper(MyStructVersion::V2(serde_json::from_slice(
                    &envelope.data().to_string().as_bytes(),
                )?))),
                3 => Ok(JsonWrapper(MyStructVersion::V3(serde_json::from_slice(
                    &envelope.data().to_string().as_bytes(),
                )?))),
                _ => Err("Unknown version".into()),
            }
        }

        fn to_versioned_envelope(&self) -> Result<Self::MsgEnvelope, Box<dyn std::error::Error>> {
            match &self.0 {
                MyStructVersion::V1(value) => Ok(JsonEnvelope {
                    version_number: 1.into(),
                    data: Cow::Owned(serde_json::to_value(value)?),
                }),
                MyStructVersion::V2(value) => Ok(JsonEnvelope {
                    version_number: 2.into(),
                    data: Cow::Owned(serde_json::to_value(value)?),
                }),
                MyStructVersion::V3(value) => Ok(JsonEnvelope {
                    version_number: 3.into(),
                    data: Cow::Owned(serde_json::to_value(value)?),
                }),
            }
        }
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
        {"version_number": 1, "field1": "value1"}
    "#;

    const UNVERSIONED_STRUCT: &'static str = r#"
        {"field1": "value1"}
    "#;

    #[test]
    fn test_promoting_unversioned_struct() -> Result<(), Box<dyn std::error::Error>> {
        let value: Cow<'static, serde_json::Value> =
            Cow::Owned(serde_json::from_str(UNVERSIONED_STRUCT)?);
        let wrapper: JsonWrapper = VersionedWrapper::deserialize(value)?;
        assert_eq!(
            wrapper,
            JsonWrapper(MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            }))
        );
        Ok(())
    }

    #[test]
    fn test_promoting_unversioned_msgpack() -> Result<(), Box<dyn std::error::Error>> {
        let value: MyStructV1 = serde_json::from_str(UNVERSIONED_STRUCT)?;
        let serialized = rmp_serde::to_vec(&value)?;
        // print serialized as hex
        println!(
            "serialized: {:?}",
            serialized
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        let serialized_wrapper = VersionedWrapper::serialize(&MyStructVersion::V1(value.clone()))?;
        println!(
            "serialized_wrapper: {:?}",
            serialized_wrapper
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        let wrapper: MyStructVersion = VersionedWrapper::deserialize(Cow::Borrowed(serialized.as_slice()))?;
        assert_eq!(
            wrapper,
            MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            })
        );
        Ok(())
    }

    #[test]
    fn upgrade_v1_to_latest() -> Result<(), Box<dyn std::error::Error>> {
        let value: Cow<'static, serde_json::Value> = Cow::Owned(serde_json::from_str(V1_STRUCT)?);
        let wrapper: JsonWrapper = VersionedWrapper::deserialize(value)?;
        assert_eq!(
            wrapper,
            JsonWrapper(MyStructVersion::V1(MyStructV1 {
                field1: "value1".to_string()
            }))
        );
        let upgraded = wrapper.0.upgrade_to_latest();
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
