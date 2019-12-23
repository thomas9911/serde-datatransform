extern crate serde;

extern crate hex;
extern crate json5;
extern crate rmp_serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml as toml_crate;

mod types;

mod error;

pub use error::SerdeTransformError;
pub use types::cbor::{
    cbor_base64_to_map, cbor_base64_urlsafe_to_map, cbor_hex_to_map, map_to_cbor_base64,
    map_to_cbor_base64_urlsafe, map_to_cbor_hex,
};
pub use types::json::{json_to_map, map_to_json};
pub use types::json5::{json5_to_map, map_to_json5};
pub use types::msgpack::{
    map_to_msgpack_base64, map_to_msgpack_base64_urlsafe, map_to_msgpack_hex,
    msgpack_base64_to_map, msgpack_base64_urlsafe_to_map, msgpack_hex_to_map,
};
pub use types::toml::{map_to_toml, toml_to_map};
pub use types::yaml::{map_to_yaml, yaml_to_map};

#[cfg(test)]
mod tests;
