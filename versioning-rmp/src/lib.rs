use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use std::io::Cursor;


fn extract_version_from_msgpack(msgpack: &[u8]) -> Result<String, rmp_serde::decode::Error> {
    struct VersionVisitor;

    impl<'de> Visitor<'de> for VersionVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expecting version string")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            while let Some(key) = map.next_key::<String>()? {
                if &key == "version" {
                    return map.next_value();
                } else {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
            Err(serde::de::Error::custom("version key not found"))
        }
    }

    let mut deserializer = rmp_serde::Deserializer::new(msgpack);
    let version = deserializer.deserialize_map(VersionVisitor)?;
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
