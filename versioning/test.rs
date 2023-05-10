#![feature(prelude_import)]
/*!# Versioning

This Rust project provides a simple and convenient way to handle versioning and upgrading of your data structures using Serde for serialization and deserialization.

# Features
- Derivable traits for chaining upgrades from older versions to the latest version of a structure.
- Generic serialization and deserialization for any supported format.
- Versioned data envelopes for partial deserialization and version-aware handling.

# Examples

Here's an example of how to use this library with a simple structure that has two versions:

```rust
use serde::{Deserialize, Serialize};
use versioning::*;
use versioning_derive::{UpgradableEnum, VersionedDeserialize, VersionedSerialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV1 {
    field1: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MyStructV2 {
    field1: String,
    new_field: String,
}

#[derive(Debug, PartialEq, UpgradableEnum, VersionedSerialize, VersionedDeserialize, Clone)]
enum MyStructVersion {
    V1(MyStructV1),
    V2(MyStructV2),
}

impl Upgrade<MyStructV2> for MyStructV1 {
    fn upgrade(self: MyStructV1) -> MyStructV2 {
        MyStructV2 {
            field1: self.field1.to_uppercase(),
            new_field: "default_value".to_string(),
        }
    }
}

// Serializing MyStructV1 to JSON format
let v1 = MyStructV1 {
    field1: "value1".to_string(),
};
let versioned_v1 = MyStructVersion::V1(v1);
let serialized_v1: serde_json::Value = MyStructVersion::serialize(&versioned_v1).unwrap();

// Deserialize MyStructVersion from JSON format
let deserialized_versioned: MyStructVersion =
    MyStructVersion::deserialize(&serialized_v1).unwrap();

// Upgrade MyStructV1 to MyStructV2
let upgraded_v2 = deserialized_versioned.upgrade_to_latest();

assert_eq!(
    upgraded_v2,
    MyStructV2 {
        field1: "VALUE1".to_string(),
        new_field: "default_value".to_string(),
    }
);
```*/
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::borrow::Cow;
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
pub struct VersionedEnvelope<T> {
    pub version_number: usize,
    pub data: T,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<T> _serde::Serialize for VersionedEnvelope<T>
    where
        T: _serde::Serialize,
    {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "VersionedEnvelope",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "version_number",
                &self.version_number,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "data",
                &self.data,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for VersionedEnvelope<T>
    where
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "version_number" => _serde::__private::Ok(__Field::__field0),
                        "data" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"version_number" => _serde::__private::Ok(__Field::__field0),
                        b"data" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<VersionedEnvelope<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = VersionedEnvelope<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct VersionedEnvelope",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        usize,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct VersionedEnvelope with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        T,
                    >(&mut __seq) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct VersionedEnvelope with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(VersionedEnvelope {
                        version_number: __field0,
                        data: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<T> = _serde::__private::None;
                    while let _serde::__private::Some(__key)
                        = match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "version_number",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        usize,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<T>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field(
                                "version_number",
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("data") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(VersionedEnvelope {
                        version_number: __field0,
                        data: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["version_number", "data"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "VersionedEnvelope",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<VersionedEnvelope<T>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for VersionedEnvelope<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "VersionedEnvelope",
            "version_number",
            &self.version_number,
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl<T> ::core::marker::StructuralPartialEq for VersionedEnvelope<T> {}
#[automatically_derived]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for VersionedEnvelope<T> {
    #[inline]
    fn eq(&self, other: &VersionedEnvelope<T>) -> bool {
        self.version_number == other.version_number && self.data == other.data
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for VersionedEnvelope<T> {
    #[inline]
    fn clone(&self) -> VersionedEnvelope<T> {
        VersionedEnvelope {
            version_number: ::core::clone::Clone::clone(&self.version_number),
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
impl SerializeFormat for serde_json::Value {
    fn versioned_serialize<T: Serialize>(
        data: T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
/// Zero copy wrapper for MessagePack bytes stored as a [std::borrow::Cow] of bytes.
pub struct MsgPackBytes<'a>(
    #[serde(with = "serde_bytes")]
    #[serde(borrow)]
    Cow<'a, [u8]>,
);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'a> _serde::Serialize for MsgPackBytes<'a> {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "MsgPackBytes",
                {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a, 'a: '__a> {
                        values: (&'__a Cow<'a, [u8]>,),
                        phantom: _serde::__private::PhantomData<MsgPackBytes<'a>>,
                    }
                    impl<'__a, 'a: '__a> _serde::Serialize
                    for __SerializeWith<'__a, 'a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            serde_bytes::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.0,),
                        phantom: _serde::__private::PhantomData::<MsgPackBytes<'a>>,
                    }
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de: 'a, 'a> _serde::Deserialize<'de> for MsgPackBytes<'a> {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de: 'a, 'a> {
                marker: _serde::__private::PhantomData<MsgPackBytes<'a>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                type Value = MsgPackBytes<'a>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct MsgPackBytes",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Cow<'a, [u8]> = match serde_bytes::deserialize(__e) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(MsgPackBytes(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match {
                        #[doc(hidden)]
                        struct __DeserializeWith<'de: 'a, 'a> {
                            value: Cow<'a, [u8]>,
                            phantom: _serde::__private::PhantomData<MsgPackBytes<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de: 'a, 'a> _serde::Deserialize<'de>
                        for __DeserializeWith<'de, 'a> {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::__private::Ok(__DeserializeWith {
                                    value: match serde_bytes::deserialize(__deserializer) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                    phantom: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                })
                            }
                        }
                        _serde::__private::Option::map(
                            match _serde::de::SeqAccess::next_element::<
                                __DeserializeWith<'de, 'a>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                            |__wrap| __wrap.value,
                        )
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct MsgPackBytes with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(MsgPackBytes(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "MsgPackBytes",
                __Visitor {
                    marker: _serde::__private::PhantomData::<MsgPackBytes<'a>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<'a> ::core::fmt::Debug for MsgPackBytes<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MsgPackBytes", &&self.0)
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for MsgPackBytes<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for MsgPackBytes<'a> {
    #[inline]
    fn eq(&self, other: &MsgPackBytes<'a>) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for MsgPackBytes<'a> {
    #[inline]
    fn clone(&self) -> MsgPackBytes<'a> {
        MsgPackBytes(::core::clone::Clone::clone(&self.0))
    }
}
impl SerializeFormat for MsgPackBytes<'_> {
    fn versioned_serialize<T: Serialize>(
        data: T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
