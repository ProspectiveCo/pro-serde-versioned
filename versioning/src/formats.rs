use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::{SerializeFormat, DeserializeFormat};

#[cfg(feature = "serde-json")]
impl SerializeFormat for serde_json::Value {
    fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::to_value(&data)?)
    }
}

#[cfg(feature = "serde-json")]
impl DeserializeFormat for serde_json::Value {
    fn versioned_deserialize<'a, T>(&'a self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Deserialize<'a>,
    {
        Ok(T::deserialize(self.clone())?)
    }
}


/// Zero copy wrapper for MessagePack bytes stored as a [std::borrow::Cow] of bytes.
#[cfg(feature = "serde-rmp")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MsgPackBytes<'a>(
    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    pub Cow<'a, [u8]>,
);

#[cfg(feature = "serde-rmp")]
impl SerializeFormat for MsgPackBytes<'_> {
    fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(MsgPackBytes(Cow::Owned(rmp_serde::to_vec(&data)?)))
    }
}

#[cfg(feature = "serde-rmp")]
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