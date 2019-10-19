extern crate serde;

extern crate hex;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SerdeTransformError {
    Cbor(String),
    Json(String),
    Yaml(String),
    Hex(String),
}

macro_rules! make_error {
    ($c:ty, $name:ident) => {
        impl From<$c> for SerdeTransformError {
            fn from(err: $c) -> SerdeTransformError {
                SerdeTransformError::$name(format!("{}", err))
            }
        }
    };
}

make_error!(hex::FromHexError, Hex);
make_error!(serde_cbor::Error, Cbor);
make_error!(serde_json::Error, Json);
make_error!(serde_yaml::Error, Yaml);

impl std::fmt::Display for SerdeTransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn json_to_map(input: &str) -> Result<serde_json::Value, SerdeTransformError> {
    let x: serde_json::Value = serde_json::from_str(input)?;
    Ok(x)
}

pub fn yaml_to_map(input: &str) -> Result<serde_yaml::Value, SerdeTransformError> {
    let x: serde_yaml::Value = serde_yaml::from_str(input)?;
    Ok(x)
}

pub fn cbor_hex_to_map(input: &str) -> Result<serde_cbor::Value, SerdeTransformError> {
    let decoded = hex::decode(input)?;
    let x: serde_cbor::Value = serde_cbor::from_reader(&decoded[..])?;
    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::{cbor_hex_to_map, json_to_map, yaml_to_map, SerdeTransformError};

    #[test]
    fn json_to_map_test() {
        let expected = serde_json::json!({
            "1": [1,2,3,4],
            "2": "oke",
            "3": {
                "nested": "item"
            }
        });

        let str_input = "{\"1\":[1,2,3,4],\"2\":\"oke\",\"3\":{\"nested\":\"item\"}}";

        assert_eq!(expected, json_to_map(str_input).unwrap())
    }

    #[test]
    fn yaml_to_map_test() {
        use serde_yaml::Value;

        let mut map = serde_yaml::Mapping::new();
        let mut list = serde_yaml::Sequence::new();
        list.push(Value::Number(serde_yaml::Number::from(1)));
        list.push(Value::Number(serde_yaml::Number::from(2)));
        list.push(Value::Number(serde_yaml::Number::from(3)));
        list.push(Value::Number(serde_yaml::Number::from(4)));
        let mut nested_map = serde_yaml::Mapping::new();
        nested_map.insert(
            Value::String(String::from("nested")),
            Value::String(String::from("item")),
        );

        map.insert(Value::String(String::from("1")), Value::Sequence(list));
        map.insert(
            Value::String(String::from("2")),
            Value::String(String::from("oke")),
        );
        map.insert(Value::String(String::from("3")), Value::Mapping(nested_map));

        let expected = Value::Mapping(map);
        let str_input = r"
'1':
  - 1
  - 2
  - 3
  - 4
'2': oke
'3':
  nested: item

        ";

        assert_eq!(expected, yaml_to_map(str_input).unwrap())
    }

    #[test]
    fn cbor_to_map_test() {
        // A3                    # map(3)
        //    61                 # text(1)
        //       31              # "1"
        //    84                 # array(4)
        //       01              # unsigned(1)
        //       02              # unsigned(2)
        //       03              # unsigned(3)
        //       04              # unsigned(4)
        //    61                 # text(1)
        //       32              # "2"
        //    63                 # text(3)
        //       6F6B65          # "oke"
        //    61                 # text(1)
        //       33              # "3"
        //    A1                 # map(1)
        //       66              # text(6)
        //          6E6573746564 # "nested"
        //       64              # text(4)
        //          6974656D     # "item"
        use serde_cbor::Value;
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();

        let list = vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(4),
        ];

        let mut nested_map = BTreeMap::new();
        nested_map.insert(
            Value::Text(String::from("nested")),
            Value::Text(String::from("item")),
        );

        map.insert(Value::Text(String::from("1")), Value::Array(list));
        map.insert(
            Value::Text(String::from("2")),
            Value::Text(String::from("oke")),
        );
        map.insert(Value::Text(String::from("3")), Value::Map(nested_map));

        let expected = Value::Map(map);

        let str_input = "A3613184010203046132636F6B656133A1666E6573746564646974656D";

        assert_eq!(expected, cbor_hex_to_map(str_input).unwrap())
    }

    #[test]
    fn json_to_map_error(){
        assert_eq!(
            Err(SerdeTransformError::Json("expected ident at line 1 column 2".to_string())),
            json_to_map("testing")
        );
    }

    #[test]
    fn yaml_to_map_error(){
        assert_eq!(
            Err(SerdeTransformError::Yaml("\"-\" is only valid inside a block at line 1 column 2".to_string())),
            yaml_to_map("{- testing}")
        );
    }

    #[test]
    fn cbor_to_map_error() {
        assert_eq!(
            Err(SerdeTransformError::Hex("Odd number of digits".to_string())),
            cbor_hex_to_map("testing")
        );
        assert_eq!(
            Err(SerdeTransformError::Cbor("unassigned type at offset 3".to_string())),
            cbor_hex_to_map("ABCDEF1234")
        );
    }
}
