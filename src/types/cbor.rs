pub fn cbor_hex_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = hex::decode(input)?;
    let x: serde_value::Value = serde_cbor::from_reader(&decoded[..])?;
    Ok(x)
}

pub fn map_to_cbor_hex(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = serde_cbor::to_vec(input)?;
    Ok(hex::encode_upper(x))
}

pub fn cbor_base64_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = base64::decode(input)?;
    let x: serde_value::Value = serde_cbor::from_reader(&decoded[..])?;
    Ok(x)
}

pub fn map_to_cbor_base64(
    input: &serde_value::Value,
) -> Result<String, crate::SerdeTransformError> {
    let x = serde_cbor::to_vec(input)?;
    Ok(base64::encode(&x))
}

pub fn cbor_base64_urlsafe_to_map(
    input: &str,
) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = base64::decode_config(input, base64::URL_SAFE)?;
    let x: serde_value::Value = serde_cbor::from_reader(&decoded[..])?;
    Ok(x)
}

pub fn map_to_cbor_base64_urlsafe(
    input: &serde_value::Value,
) -> Result<String, crate::SerdeTransformError> {
    let x = serde_cbor::to_vec(input)?;
    Ok(base64::encode_config(&x, base64::URL_SAFE))
}
