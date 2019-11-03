pub fn toml_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let x: serde_value::Value = toml_crate::from_str(input)?;
    Ok(x)
}

pub fn map_to_toml(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = toml_crate::to_string(input)?;
    Ok(x)
}
