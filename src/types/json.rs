pub fn json_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let x: serde_value::Value = serde_json::from_str(input)?;
    Ok(x)
}

pub fn map_to_json(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = serde_json::to_string(input)?;
    Ok(x)
}
