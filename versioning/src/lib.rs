use serde::{Deserialize, Serialize};
use serde_versioning_derive::UpgradableEnum;

pub trait Upgrade<To> {
    fn upgrade(self) -> To;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyStructV1 {
    field1: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyStructV2 {
    field1: String,
    new_field: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    #[latest]
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
            second_new_field: "default_value".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1_STRUCT: &'static str = r#"
        {"version": "V1", "field1": "value1"}
    "#;

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
        let upgraded: MyStructV3 = result.upgrade_to_latest();
        assert_eq!(
            upgraded,
            MyStructV3 {
                field1: "VALUE1".to_string(),
                new_field: "default_value".to_string(),
                second_new_field: "default_value".to_string()
            }
        );
        Ok(())
    }
}
