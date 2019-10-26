
pub fn cbor_hex_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = hex::decode(input)?;
    let x: serde_value::Value = serde_cbor::from_reader(&decoded[..])?;
    Ok(x)
}



pub fn map_to_cbor_hex(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = serde_cbor::to_vec(input)?;
    Ok(hex::encode_upper(x))
}