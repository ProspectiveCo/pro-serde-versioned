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

#![doc = include_str!("../README.md")]

mod formats;

#[cfg(feature = "derive")]
pub use pro_serde_versioned_derive::{VersionedDeserialize, VersionedSerialize, VersionedUpgrade};
use serde::{Deserialize, Serialize};

pub use crate::formats::*;

/// Derivable trait used to chain upgrade a versioned wrapper to the latest
/// version of a structure (e.g. v1 -> v2 -> ... -> latest)
pub trait VersionedUpgrade {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}

/// Defines the next version of a given upgradable type (e.g. mystructv1 ->
/// mystructv1)
pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

/// Allows for serializing to any supported format.
pub trait VersionedSerialize {
    type VersionedEnvelope<F: Serialize>: Serialize;

    fn to_envelope<F>(&self) -> Result<Self::VersionedEnvelope<F>, F::Error>
    where
        F: SerializeFormat;

    fn versioned_serialize<F>(&self) -> Result<F, F::Error>
    where
        F: SerializeFormat,
    {
        Ok(F::serialize_format(self.to_envelope::<F>()?)?)
    }
}

/// Allows for serializing from any supported format.
pub trait VersionedDeserialize: Sized + Clone {
    type VersionedEnvelope<'a, F: Deserialize<'a>>: Deserialize<'a>;

    fn from_envelope<'a, F>(data: &Self::VersionedEnvelope<'a, F>) -> Result<Self, F::Error>
    where
        F: DeserializeFormat + Deserialize<'a>;

    fn versioned_deserialize<'a, F>(data: &'a F) -> Result<Self, F::Error>
    where
        F: DeserializeFormat + Deserialize<'a>,
    {
        let envelope: Self::VersionedEnvelope<'a, F> = F::deserialize_format(data)?;
        Self::from_envelope(&envelope)
    }
}

/// Serialize to the underlying format of a given serialization standard. (e.g.
/// [serde_json::Value] for JSON, [std::borrow::Cow] of bytes for MsgPack, etc.)
pub trait SerializeFormat: Sized + Serialize {
    type Error: serde::ser::Error;
    fn serialize_format<T>(data: T) -> Result<Self, Self::Error>
    where
        T: Serialize;
}

/// Deserialize from the underlying format of a given serialization standard.
/// (e.g. [serde_json::Value] for JSON, [std::borrow::Cow] of bytes for MsgPack,
/// etc.)
pub trait DeserializeFormat: Sized {
    type Error: serde::de::Error;
    fn deserialize_format<'a, T>(&'a self) -> Result<T, Self::Error>
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
