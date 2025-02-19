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
    fn json5_to_map_test() {
        let list = vec![
            serde_value::Value::I64(1),
            serde_value::Value::I64(-2),
            serde_value::Value::I64(3),
            serde_value::Value::I64(4),
        ];

        let expected = var_data(list);

        let str_input = "{\"1\":[1,-2,3,+4,],\"2\":\"oke\",\"3\":{nested: 'item'}}";

        assert_eq!(expected, json5_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_json5_test() {
        let value_input = data();

        let expected = String::from("{\"1\":[1,2,3,4],\"2\":\"oke\",\"3\":{\"nested\":\"item\"}}");

        assert_eq!(expected, map_to_json5(&value_input).unwrap())
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
    fn map_to_yaml_test() {
        let value_input = data();

        let expected = r#"'1':
- 1
- 2
- 3
- 4
'2': oke
'3':
  nested: item
"#;

        assert_eq!(expected, map_to_yaml(&value_input).unwrap())
    }

    #[test]
    fn cbor_hex_to_map_test() {
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
    fn map_to_cbor_hex_test() {
        let value_input = data();

        let expected = "A3613184010203046132636F6B656133A1666E6573746564646974656D";
        assert_eq!(expected, map_to_cbor_hex(&value_input).unwrap())
    }

    #[test]
    fn cbor_base64_to_map_test() {
        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "o2ExhAECAwRhMmNva2VhM6FmbmVzdGVkZGl0ZW0";

        assert_eq!(expected, cbor_base64_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_cbor_base64_test() {
        let value_input = data();

        let expected = "o2ExhAECAwRhMmNva2VhM6FmbmVzdGVkZGl0ZW0=";
        assert_eq!(expected, map_to_cbor_base64(&value_input).unwrap())
    }

    #[test]
    fn cbor_base64_urlsafe_to_map_test() {
        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "o2ExhAECAwRhMmNva2VhM6FmbmVzdGVkZGl0ZW0";

        assert_eq!(expected, cbor_base64_urlsafe_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_cbor_base64_urlsafe_test() {
        let value_input = data();

        let expected = "o2ExhAECAwRhMmNva2VhM6FmbmVzdGVkZGl0ZW0=";
        assert_eq!(expected, map_to_cbor_base64_urlsafe(&value_input).unwrap())
    }

    #[test]
    fn msgpack_hex_to_map_test() {
        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "83A1319401020304A132A36F6B65A13381A66E6573746564A46974656D";
        assert_eq!(expected, msgpack_hex_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_msgpack_hex_test() {
        let value_input = data();

        let expected = "83A1319401020304A132A36F6B65A13381A66E6573746564A46974656D";
        assert_eq!(expected, map_to_msgpack_hex(&value_input).unwrap())
    }

    #[test]
    fn msgpack_base64_to_map_test() {
        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "g6ExlAECAwShMqNva2WhM4GmbmVzdGVkpGl0ZW0=";
        assert_eq!(expected, msgpack_base64_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_msgpack_base64_test() {
        let value_input = data();

        let expected = "g6ExlAECAwShMqNva2WhM4GmbmVzdGVkpGl0ZW0=";
        assert_eq!(expected, map_to_msgpack_base64(&value_input).unwrap())
    }

    #[test]
    fn msgpack_base64_urlsafe_to_map_test() {
        let list = vec![
            serde_value::Value::U8(1),
            serde_value::Value::U8(2),
            serde_value::Value::U8(3),
            serde_value::Value::U8(4),
        ];

        let expected = var_data(list);

        let str_input = "g6ExlAECAwShMqNva2WhM4GmbmVzdGVkpGl0ZW0=";
        assert_eq!(expected, msgpack_base64_urlsafe_to_map(str_input).unwrap())
    }

    #[test]
    fn map_to_msgpack_base64_urlsafe_test() {
        let value_input = data();

        let expected = "g6ExlAECAwShMqNva2WhM4GmbmVzdGVkpGl0ZW0=";
        assert_eq!(
            expected,
            map_to_msgpack_base64_urlsafe(&value_input).unwrap()
        )
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
    fn json5_to_map_error() {
        assert_eq!(
            Err(
                SerdeTransformError::Json5(
                    String::from(
                        " --> 1:1\n  |\n1 | testing\n  | ^---\n  |\n  = expected array, boolean, null, number, object, or string"
                    )
                )
            ),
            json5_to_map("testing")
        );
    }

    #[test]
    fn yaml_to_map_error() {
        assert_eq!(
            Err(SerdeTransformError::Yaml(
                "did not find expected node content at line 1 column 2, while parsing a flow node"
                    .to_string()
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
