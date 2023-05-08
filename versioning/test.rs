#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::borrow::Cow;
use serde::{de::Visitor, Deserialize, Serialize};
trait UpgradableEnum {
    type Latest;
    fn upgrade_to_latest(self) -> Self::Latest;
}
pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}
pub struct VersionNumber(usize);
#[automatically_derived]
impl ::core::marker::Copy for VersionNumber {}
#[automatically_derived]
impl ::core::clone::Clone for VersionNumber {
    #[inline]
    fn clone(&self) -> VersionNumber {
        let _: ::core::clone::AssertParamIsClone<usize>;
        *self
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for VersionNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VersionNumber", &&self.0)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for VersionNumber {}
#[automatically_derived]
impl ::core::cmp::PartialEq for VersionNumber {
    #[inline]
    fn eq(&self, other: &VersionNumber) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for VersionNumber {}
#[automatically_derived]
impl ::core::cmp::Eq for VersionNumber {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<usize>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for VersionNumber {
    #[inline]
    fn partial_cmp(
        &self,
        other: &VersionNumber,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for VersionNumber {
    #[inline]
    fn cmp(&self, other: &VersionNumber) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for VersionNumber {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "VersionNumber",
                &self.0,
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
    impl<'de> _serde::Deserialize<'de> for VersionNumber {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<VersionNumber>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VersionNumber;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct VersionNumber",
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
                    let __field0: usize = match <usize as _serde::Deserialize>::deserialize(
                        __e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(VersionNumber(__field0))
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
                                    &"tuple struct VersionNumber with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(VersionNumber(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "VersionNumber",
                __Visitor {
                    marker: _serde::__private::PhantomData::<VersionNumber>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
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
    fn to_versioned_envelope(
        &self,
    ) -> Result<Self::MsgEnvelope, Box<dyn std::error::Error>>;
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
pub struct DataWrapper<'a>(Cow<'a, [u8]>);
#[automatically_derived]
impl<'a> ::core::fmt::Debug for DataWrapper<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DataWrapper", &&self.0)
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for DataWrapper<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for DataWrapper<'a> {
    #[inline]
    fn eq(&self, other: &DataWrapper<'a>) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DataWrapper<'a> {
    #[inline]
    fn clone(&self) -> DataWrapper<'a> {
        DataWrapper(::core::clone::Clone::clone(&self.0))
    }
}
struct DataWrapperVisitor;
impl<'a> Visitor<'a> for DataWrapperVisitor {
    type Value = DataWrapper<'a>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("byte array")
    }
    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(DataWrapper(Cow::Borrowed(v)))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(DataWrapper(Cow::Owned(v.to_owned())))
    }
}
impl<'a> Deserialize<'a> for DataWrapper<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_bytes(DataWrapperVisitor)
    }
}
impl<'a> Serialize for DataWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}
pub trait Envelope<'a>: Sized + Clone {
    type Data;
    fn version_number(&'a self) -> VersionNumber;
    fn data(&'a self) -> Self::Data;
    fn deserialize(data: Self::Data) -> Result<Self, Box<dyn std::error::Error>>;
    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>>;
}
pub struct MsgPackEnvelope<'a> {
    version_number: VersionNumber,
    #[serde(with = "serde_bytes")]
    data: Cow<'a, [u8]>,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, 'a> _serde::Deserialize<'de> for MsgPackEnvelope<'a> {
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
            struct __Visitor<'de, 'a> {
                marker: _serde::__private::PhantomData<MsgPackEnvelope<'a>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                type Value = MsgPackEnvelope<'a>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct MsgPackEnvelope",
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
                        VersionNumber,
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
                                    &"struct MsgPackEnvelope with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match {
                        #[doc(hidden)]
                        struct __DeserializeWith<'de, 'a> {
                            value: Cow<'a, [u8]>,
                            phantom: _serde::__private::PhantomData<MsgPackEnvelope<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de, 'a> _serde::Deserialize<'de>
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
                                    1usize,
                                    &"struct MsgPackEnvelope with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(MsgPackEnvelope {
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
                    let mut __field0: _serde::__private::Option<VersionNumber> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Cow<'a, [u8]>> = _serde::__private::None;
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
                                        VersionNumber,
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
                                __field1 = _serde::__private::Some({
                                    #[doc(hidden)]
                                    struct __DeserializeWith<'de, 'a> {
                                        value: Cow<'a, [u8]>,
                                        phantom: _serde::__private::PhantomData<
                                            MsgPackEnvelope<'a>,
                                        >,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de, 'a> _serde::Deserialize<'de>
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
                                    match _serde::de::MapAccess::next_value::<
                                        __DeserializeWith<'de, 'a>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__wrapper) => __wrapper.value,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                });
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
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    _serde::__private::Ok(MsgPackEnvelope {
                        version_number: __field0,
                        data: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["version_number", "data"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "MsgPackEnvelope",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MsgPackEnvelope<'a>>,
                    lifetime: _serde::__private::PhantomData,
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
    impl<'a> _serde::Serialize for MsgPackEnvelope<'a> {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "MsgPackEnvelope",
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
                {
                    #[doc(hidden)]
                    struct __SerializeWith<'__a, 'a: '__a> {
                        values: (&'__a Cow<'a, [u8]>,),
                        phantom: _serde::__private::PhantomData<MsgPackEnvelope<'a>>,
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
                        values: (&self.data,),
                        phantom: _serde::__private::PhantomData::<MsgPackEnvelope<'a>>,
                    }
                },
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
#[automatically_derived]
impl<'a> ::core::fmt::Debug for MsgPackEnvelope<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MsgPackEnvelope",
            "version_number",
            &self.version_number,
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for MsgPackEnvelope<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for MsgPackEnvelope<'a> {
    #[inline]
    fn eq(&self, other: &MsgPackEnvelope<'a>) -> bool {
        self.version_number == other.version_number && self.data == other.data
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for MsgPackEnvelope<'a> {
    #[inline]
    fn clone(&self) -> MsgPackEnvelope<'a> {
        MsgPackEnvelope {
            version_number: ::core::clone::Clone::clone(&self.version_number),
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
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
        Ok(rmp_serde::from_slice(&data)?)
    }
    fn serialize(&self) -> Result<Self::Data, Box<dyn std::error::Error>> {
        Ok(Cow::Owned(rmp_serde::to_vec(self)?))
    }
}
pub struct JsonEnvelope<'a> {
    #[serde(default)]
    version_number: VersionNumber,
    #[serde(flatten)]
    data: Cow<'a, serde_json::Value>,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'a> _serde::Serialize for JsonEnvelope<'a> {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_map(
                __serializer,
                _serde::__private::None,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeMap::serialize_entry(
                &mut __serde_state,
                "version_number",
                &self.version_number,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::Serialize::serialize(
                &&self.data,
                _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeMap::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, 'a> _serde::Deserialize<'de> for JsonEnvelope<'a> {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field<'de> {
                __field0,
                __other(_serde::__private::de::Content<'de>),
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_bool<__E>(
                    self,
                    __value: bool,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::Bool(__value)),
                    )
                }
                fn visit_i8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::I8(__value)),
                    )
                }
                fn visit_i16<__E>(
                    self,
                    __value: i16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::I16(__value)),
                    )
                }
                fn visit_i32<__E>(
                    self,
                    __value: i32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::I32(__value)),
                    )
                }
                fn visit_i64<__E>(
                    self,
                    __value: i64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::I64(__value)),
                    )
                }
                fn visit_u8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::U8(__value)),
                    )
                }
                fn visit_u16<__E>(
                    self,
                    __value: u16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::U16(__value)),
                    )
                }
                fn visit_u32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::U32(__value)),
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::U64(__value)),
                    )
                }
                fn visit_f32<__E>(
                    self,
                    __value: f32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::F32(__value)),
                    )
                }
                fn visit_f64<__E>(
                    self,
                    __value: f64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::F64(__value)),
                    )
                }
                fn visit_char<__E>(
                    self,
                    __value: char,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::Char(__value)),
                    )
                }
                fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(
                        __Field::__other(_serde::__private::de::Content::Unit),
                    )
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
                        _ => {
                            let __value = _serde::__private::de::Content::String(
                                _serde::__private::ToString::to_string(__value),
                            );
                            _serde::__private::Ok(__Field::__other(__value))
                        }
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
                        _ => {
                            let __value = _serde::__private::de::Content::ByteBuf(
                                __value.to_vec(),
                            );
                            _serde::__private::Ok(__Field::__other(__value))
                        }
                    }
                }
                fn visit_borrowed_str<__E>(
                    self,
                    __value: &'de str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "version_number" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = _serde::__private::de::Content::Str(__value);
                            _serde::__private::Ok(__Field::__other(__value))
                        }
                    }
                }
                fn visit_borrowed_bytes<__E>(
                    self,
                    __value: &'de [u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"version_number" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = _serde::__private::de::Content::Bytes(__value);
                            _serde::__private::Ok(__Field::__other(__value))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
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
            struct __Visitor<'de, 'a> {
                marker: _serde::__private::PhantomData<JsonEnvelope<'a>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                type Value = JsonEnvelope<'a>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct JsonEnvelope",
                    )
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<VersionNumber> = _serde::__private::None;
                    let mut __collect = _serde::__private::Vec::<
                        _serde::__private::Option<
                            (
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            ),
                        >,
                    >::new();
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
                                        VersionNumber,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__other(__name) => {
                                __collect
                                    .push(
                                        _serde::__private::Some((
                                            __name,
                                            match _serde::de::MapAccess::next_value(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        )),
                                    );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::Default::default(),
                    };
                    let __field1: Cow<'a, serde_json::Value> = match _serde::de::Deserialize::deserialize(
                        _serde::__private::de::FlatMapDeserializer(
                            &mut __collect,
                            _serde::__private::PhantomData,
                        ),
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(JsonEnvelope {
                        version_number: __field0,
                        data: __field1,
                    })
                }
            }
            _serde::Deserializer::deserialize_map(
                __deserializer,
                __Visitor {
                    marker: _serde::__private::PhantomData::<JsonEnvelope<'a>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<'a> ::core::fmt::Debug for JsonEnvelope<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JsonEnvelope",
            "version_number",
            &self.version_number,
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl<'a> ::core::marker::StructuralPartialEq for JsonEnvelope<'a> {}
#[automatically_derived]
impl<'a> ::core::cmp::PartialEq for JsonEnvelope<'a> {
    #[inline]
    fn eq(&self, other: &JsonEnvelope<'a>) -> bool {
        self.version_number == other.version_number && self.data == other.data
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for JsonEnvelope<'a> {
    #[inline]
    fn clone(&self) -> JsonEnvelope<'a> {
        JsonEnvelope {
            version_number: ::core::clone::Clone::clone(&self.version_number),
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
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
