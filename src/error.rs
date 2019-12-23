use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SerdeTransformError {
    Cbor(String),
    Json(String),
    Json5(String),
    Toml(String),
    Yaml(String),
    Hex(String),
    MsgPack(String),
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
make_error!(json5::Error, Json5);
make_error!(toml_crate::ser::Error, Toml);
make_error!(toml_crate::de::Error, Toml);
make_error!(serde_yaml::Error, Yaml);
make_error!(rmp_serde::encode::Error, MsgPack);
make_error!(rmp_serde::decode::Error, MsgPack);

impl std::fmt::Display for SerdeTransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn kaas() {
    let err = SerdeTransformError::Cbor(String::from("this goes wrong"));
    assert_eq!(
        String::from("Cbor(\"this goes wrong\")"),
        format!("{}", err)
    );
}
