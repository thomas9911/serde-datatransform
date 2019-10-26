
pub fn yaml_to_map(input: &str) -> Result<serde_value::Value, crate::SerdeTransformError> {
    let x: serde_value::Value = serde_yaml::from_str(input)?;
    Ok(x)
}


pub fn map_to_yaml(input: &serde_value::Value) -> Result<String, crate::SerdeTransformError> {
    let x = serde_yaml::to_string(input)?;
    Ok(x)
}
