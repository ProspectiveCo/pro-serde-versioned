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

pub trait VersionedWrapper<'a, Format: VersionedSerde<'a>>: VersionedWrapperSer<'a, Format> + VersionedWrapperDe<'a, Format> {}

impl <'a, Format: VersionedSerde<'a>, T: VersionedWrapperSer<'a, Format> + VersionedWrapperDe<'a, Format>> VersionedWrapper<'a, Format> for T {}

pub trait VersionedWrapperSer<'a, Format: VersionedSerde<'a>>: Sized + Clone {
    fn to_versioned_envelope(
        &self,
    ) -> Result<VersionedEnvelope<Format>, Box<dyn std::error::Error>>;

    fn serialize(&self) -> Result<Format, Box<dyn std::error::Error>> {
        Format::versioned_serialize(self.to_versioned_envelope()?)
    }
}

pub trait VersionedWrapperDe<'a, Format: VersionedSerde<'a>>: Sized + Clone {
    fn from_versioned_envelope(
        envelope: VersionedEnvelope<Format>,
    ) -> Result<Self, Box<dyn std::error::Error>>;

    fn deserialize(data: Format) -> Result<Self, Box<dyn std::error::Error>> {
        let envelope = Format::versioned_deserialize(data)?;
        Self::from_versioned_envelope(envelope)
    }
}


pub trait VersionedSerde<'a>: VersionedSer + VersionedDe<'a> {}

impl <'a, T: VersionedSer + VersionedDe<'a>> VersionedSerde<'a> for T {}

pub trait VersionedSer: Sized {
    fn versioned_serialize(
        data: VersionedEnvelope<Self>,
    ) -> Result<Self, Box<dyn std::error::Error>>;
}

pub trait VersionedDe<'a>: Sized {
    fn versioned_deserialize(self) -> Result<VersionedEnvelope<Self>, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MsgPackBytes<'a>(
    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    Cow<'a, [u8]>,
);

impl VersionedSer for MsgPackBytes<'_> {
    fn versioned_serialize(
        data: VersionedEnvelope<Self>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(MsgPackBytes(Cow::Owned(rmp_serde::to_vec(&data)?)))
    }
}

impl<'a> VersionedDe<'a> for MsgPackBytes<'a> {
    fn versioned_deserialize(self) -> Result<VersionedEnvelope<Self>, Box<dyn std::error::Error>> {
        match self.0 {
            Cow::Borrowed(bytes) => Ok(rmp_serde::from_slice(bytes)?),
            Cow::Owned(bytes) => {
                let borrowed_envelope: VersionedEnvelope<MsgPackBytes<'_>> =
                    rmp_serde::from_slice(&bytes)?;
                let owned: VersionedEnvelope<Self> = VersionedEnvelope {
                    version_number: borrowed_envelope.version_number,
                    data: MsgPackBytes(match borrowed_envelope.data.0 {
                        Cow::Borrowed(bytes) => Cow::Owned(bytes.to_vec()),
                        Cow::Owned(bytes) => Cow::Owned(bytes),
                    }),
                };
                Ok(owned)
            }
        }
    }
}

impl VersionedSer for serde_json::Value {
    fn versioned_serialize(
        data: VersionedEnvelope<Self>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::to_value(&data)?)
    }
}

impl VersionedDe<'_> for serde_json::Value {
    fn versioned_deserialize(self) -> Result<VersionedEnvelope<Self>, Box<dyn std::error::Error>> {
        Ok(serde_json::from_value(self.clone())?)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VersionedEnvelope<T> {
    version_number: VersionNumber,
    data: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    use versioning_derive::{UpgradableEnum, VersionedWrapper};

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

    #[derive(Debug, PartialEq, UpgradableEnum, VersionedWrapper, Clone)]
    #[versioned_wrapper(formats(msgpack, json))]
    pub enum MyStructVersion {
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

        let wrapper: MyStructVersion = MyStructVersion::deserialize(value.clone())?;

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
        let versioned_struct: MyStructVersion = MyStructVersion::deserialize(json_value)?;
        let serialized_wrapper: MsgPackBytes = MyStructVersion::serialize(&versioned_struct)?;

        // We want this to be a borrow so that the msgpack deserialize is zero copy
        let serialized_wrapper = serialized_wrapper.0.as_ref().into();

        assert!(match serialized_wrapper {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false,
        });

        let hex = 
            serialized_wrapper
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");

        assert_eq!(
            hex,
            "92 01 c4 08 91 a6 76 61 6c 75 65 31"
        );

        // Asserting that serializer is symmetric
        let _: MyStructVersion = MyStructVersion::deserialize(MsgPackBytes(serialized_wrapper))?;

        Ok(())
    }

}
