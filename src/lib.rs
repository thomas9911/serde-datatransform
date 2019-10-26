extern crate serde;

extern crate hex;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml as toml_crate;

mod types;

mod error;

pub use error::SerdeTransformError;
pub use types::cbor::{cbor_hex_to_map, map_to_cbor_hex};
pub use types::json::{json_to_map, map_to_json};
pub use types::toml::{map_to_toml, toml_to_map};
pub use types::yaml::{yaml_to_map, map_to_yaml};

#[cfg(test)]
mod tests {
    use crate::*;

    fn var_data(vec_list: Vec<serde_value::Value>) -> serde_value::Value {
        use serde_value::Value;
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();

        let mut nested_map = BTreeMap::new();
        nested_map.insert(
            Value::String(String::from("nested")),
            Value::String(String::from("item")),
        );

        map.insert(Value::String(String::from("1")), Value::Seq(vec_list));
        map.insert(
            Value::String(String::from("2")),
            Value::String(String::from("oke")),
        );
        map.insert(Value::String(String::from("3")), Value::Map(nested_map));

        let expected = Value::Map(map);
        expected
    }

    fn data() -> serde_value::Value {
        use serde_value::Value;

        let mut list = Vec::new();
        list.push(Value::U64(1));
        list.push(Value::U64(2));
        list.push(Value::U64(3));
        list.push(Value::U64(4));
        var_data(list)
    }

    #[test]
    fn json_to_map_test() {
        let expected = data();

        let str_input = "{\"1\":[1,2,3,4],\"2\":\"oke\",\"3\":{\"nested\":\"item\"}}";

        assert_eq!(expected, json_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_json_test() {
        let value_input = data();

        let expected = String::from("{\"1\":[1,2,3,4],\"2\":\"oke\",\"3\":{\"nested\":\"item\"}}");

        assert_eq!(expected, map_to_json(&value_input).unwrap())
    }

    #[test]
    fn toml_to_map_test() {
        let str_input = r#"
                    1 = [1,2,3,4]
                    2 = "oke"
                    [3]
                    nested = "item"
                    "#;

        let list = vec![
            serde_value::Value::I64(1),
            serde_value::Value::I64(2),
            serde_value::Value::I64(3),
            serde_value::Value::I64(4),
        ];

        let expected = var_data(list);

        assert_eq!(expected, toml_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_toml_test() {
        let value_input = data();

        let expected = String::from(
            r#"1 = [1, 2, 3, 4]
2 = "oke"

[3]
nested = "item"
"#,
        );

        assert_eq!(expected, map_to_toml(&value_input).unwrap())
    }

    #[test]
    fn yaml_to_map_test() {
        let expected = data();

        let str_input = r#"
'1':
  - 1
  - 2
  - 3
  - 4
'2': oke
'3':
  nested: item

        "#;

        assert_eq!(expected, yaml_to_map(str_input).unwrap())
    }


    #[test]
    fn map_to_yaml_test(){
        let value_input = data();

        let expected = r#"---
"1":
  - 1
  - 2
  - 3
  - 4
"2": oke
"3":
  nested: item"#;

        assert_eq!(expected, map_to_yaml(&value_input).unwrap())
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

        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "A3613184010203046132636F6B656133A1666E6573746564646974656D";

        assert_eq!(expected, cbor_hex_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_cbor_hex_test(){
        let value_input = data();

        let expected = "A3613184010203046132636F6B656133A1666E6573746564646974656D";
        assert_eq!(expected, map_to_cbor_hex(&value_input).unwrap())

    }

    #[test]
    fn json_to_map_error() {
        assert_eq!(
            Err(SerdeTransformError::Json(
                "expected ident at line 1 column 2".to_string()
            )),
            json_to_map("testing")
        );
    }

    #[test]
    fn yaml_to_map_error() {
        assert_eq!(
            Err(SerdeTransformError::Yaml(
                "\"-\" is only valid inside a block at line 1 column 2".to_string()
            )),
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
            Err(SerdeTransformError::Cbor(
                "unassigned type at offset 3".to_string()
            )),
            cbor_hex_to_map("ABCDEF1234")
        );
    }
}
