pub fn msgpack_hex_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = hex::decode(input)?;
    let x: serde_value::Value = rmp_serde::from_read_ref(&decoded[..])?;
    Ok(x)
}

pub fn map_to_msgpack_hex(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = rmp_serde::to_vec(input)?;
    Ok(hex::encode_upper(x))
}
