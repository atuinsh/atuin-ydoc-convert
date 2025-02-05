mod block;
mod content;
mod converter;

pub use converter::Error;

pub fn convert_to_value(xml: String) -> Result<serde_json::Value, Error> {
    converter::convert(xml)
}

pub fn convert_to_json(xml: String) -> Result<String, Error> {
    let val = convert_to_value(xml)?;
    Ok(serde_json::to_string(&val).unwrap())
}

pub fn convert_to_json_pretty(xml: String) -> Result<String, Error> {
    let val = convert_to_value(xml)?;
    Ok(serde_json::to_string_pretty(&val).unwrap())
}
