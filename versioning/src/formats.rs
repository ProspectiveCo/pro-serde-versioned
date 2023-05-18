use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{DeserializeFormat, SerializeFormat};

#[cfg(feature = "serde_json")]
impl SerializeFormat for serde_json::Value {
    type Error = serde_json::Error;

    fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Self::Error> {
        Ok(serde_json::to_value(&data)?)
    }
}

#[cfg(feature = "serde_json")]
impl DeserializeFormat for serde_json::Value {
    type Error = serde_json::Error;

    fn versioned_deserialize<'a, T>(&'a self) -> Result<T, Self::Error>
    where
        T: Deserialize<'a>,
    {
        Ok(T::deserialize(self.clone())?)
    }
}

/// Zero copy wrapper for MessagePack bytes stored as a [std::borrow::Cow] of bytes.
#[cfg(feature = "serde_rmp")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MsgPackBytes<'a>(
    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    pub Cow<'a, [u8]>,
);

#[cfg(feature = "serde_rmp")]
impl SerializeFormat for MsgPackBytes<'_> {
    type Error = rmp_serde::encode::Error;
    fn versioned_serialize<T: Serialize>(data: T) -> Result<Self, Self::Error> {
        Ok(MsgPackBytes(Cow::Owned(rmp_serde::to_vec(&data)?)))
    }
}

#[cfg(feature = "serde_rmp")]
impl<'a> DeserializeFormat for MsgPackBytes<'a> {
    type Error = rmp_serde::decode::Error;
    fn versioned_deserialize<'b, T: Deserialize<'b>>(&'b self) -> Result<T, Self::Error> {
        match &self.0 {
            Cow::Borrowed(bytes) => Ok(rmp_serde::from_slice(bytes)?),
            Cow::Owned(bytes) => Ok(rmp_serde::from_slice(&bytes)?),
        }
    }
}
