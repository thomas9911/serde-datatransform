pub fn json5_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let x: serde_value::Value = json5::from_str(input)?;
    Ok(x)
}

pub fn map_to_json5(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = json5::to_string(input)?;
    Ok(x)
}
