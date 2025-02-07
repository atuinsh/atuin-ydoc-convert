mod block;
mod content;
mod converter;
mod serialize;

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

pub fn get_fragment_xml(doc: yrs::Doc, fragment_name: String) -> String {
    serialize::get_fragment_xml(doc, fragment_name)
}
