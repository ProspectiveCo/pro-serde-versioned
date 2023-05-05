use std::borrow::Cow;

use serde::{Deserialize, Serialize};

trait UpgradableEnum {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}

pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

pub trait VersionedWrapper<'a>: Sized {
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

pub trait Envelope<'a>: Sized {
    type Data;
    fn version_number(&'a self) -> usize;
    fn data(&'a self) -> Self::Data;
    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>>;
    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MsgPackEnvelope<'a> {
    version_number: usize,
    data: Cow<'a, [u8]>,
}

impl<'a> Envelope<'a> for MsgPackEnvelope<'a> {
    type Data = Cow<'a, [u8]>;

    fn version_number(&'a self) -> usize {
        self.version_number
    }

    fn data(&'a self) -> Self::Data {
        self.data.clone()
    }

    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(rmp_serde::from_slice(&data)?)
    }

    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>> {
        Ok(rmp_serde::to_vec(self)?.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonEnvelope<'a> {
    #[serde(default)]
    version_number: usize,
    #[serde(flatten)]
    data: Cow<'a, serde_json::Value>,
}

impl<'a> Envelope<'a> for JsonEnvelope<'a> {
    type Data = Cow<'a, serde_json::Value>;

    fn version_number(&'a self) -> usize {
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

    #[derive(Debug, PartialEq, UpgradableEnum)]
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
            match envelope.version_number() {
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
                        version_number: 1,
                        data: struct_ser.into_inner().into(),
                    })
                }
                MyStructVersion::V2(value) => {
                    let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                    value.serialize(&mut struct_ser)?;
                    Ok(MsgPackEnvelope {
                        version_number: 2,
                        data: struct_ser.into_inner().into(),
                    })
                }
                MyStructVersion::V3(value) => {
                    let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                    value.serialize(&mut struct_ser)?;
                    Ok(MsgPackEnvelope {
                        version_number: 3,
                        data: struct_ser.into_inner().into(),
                    })
                }
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct JsonWrapper(MyStructVersion);

    impl<'a> VersionedWrapper<'a> for JsonWrapper {
        type MsgEnvelope = JsonEnvelope<'a>;

        fn from_versioned_envelope(
            envelope: Self::MsgEnvelope,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            match envelope.version_number() {
                0 => Ok(JsonWrapper(MyStructVersion::V1(serde_json::from_slice(
                    &envelope.data().to_string().as_bytes(),
                )?))),
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
                    version_number: 1,
                    data: Cow::Owned(serde_json::to_value(value)?),
                }),
                MyStructVersion::V2(value) => Ok(JsonEnvelope {
                    version_number: 2,
                    data: Cow::Owned(serde_json::to_value(value)?),
                }),
                MyStructVersion::V3(value) => Ok(JsonEnvelope {
                    version_number: 3,
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
