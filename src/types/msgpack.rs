pub fn msgpack_hex_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = hex::decode(input)?;
    let x: serde_value::Value = rmp_serde::from_read_ref(&decoded[..])?;
    Ok(x)
}

pub fn map_to_msgpack_hex(
    input: &serde_value::Value,
) -> Result<String, crate::SerdeTransformError> {
    let x = rmp_serde::to_vec(input)?;
    Ok(hex::encode_upper(x))
}

pub fn msgpack_base64_to_map(
    input: &str,
) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = base64::decode(input)?;
    let x: serde_value::Value = rmp_serde::from_read_ref(&decoded[..])?;
    Ok(x)
}

pub fn map_to_msgpack_base64(
    input: &serde_value::Value,
) -> Result<String, crate::SerdeTransformError> {
    let x = rmp_serde::to_vec(input)?;
    Ok(base64::encode(&x))
}

pub fn msgpack_base64_urlsafe_to_map(
    input: &str,
) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let decoded = base64::decode_config(input, base64::URL_SAFE)?;
    let x: serde_value::Value = rmp_serde::from_read_ref(&decoded[..])?;
    Ok(x)
}

pub fn map_to_msgpack_base64_urlsafe(
    input: &serde_value::Value,
) -> Result<String, crate::SerdeTransformError> {
    let x = rmp_serde::to_vec(input)?;
    Ok(base64::encode_config(&x, base64::URL_SAFE))
}
