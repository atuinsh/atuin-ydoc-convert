use roxmltree::Attributes;
use serde_json::{json, Value};

use super::style::Style;

#[derive(Clone, Debug)]
pub(crate) struct BasicContent {
    pub type_name: String,
    pub props: serde_json::Map<String, Value>,
    pub content: Option<Vec<BasicContent>>,
    pub styles: Vec<Style>,
}

impl BasicContent {
    pub fn new() -> Self {
        BasicContent {
            type_name: String::new(),
            content: None,
            props: serde_json::Map::new(),
            styles: Vec::new(),
        }
    }

    pub fn apply_attributes(&mut self, attributes: Attributes) {
        for attr in attributes {
            self.props
                .insert(attr.name().to_string(), json!(attr.value()));
        }
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), json!(self.type_name));
        let mut styles = serde_json::Map::new();
        for style in self.styles.iter() {
            styles.insert(style.name().to_string(), json!(true));
        }
        map.insert("styles".to_string(), Value::Object(styles));
        for prop in self.props.iter() {
            map.insert(prop.0.clone(), prop.1.clone());
        }
        if let Some(content) = &self.content {
            map.insert(
                "content".to_string(),
                json!(content.iter().map(|c| c.to_json()).collect::<Vec<Value>>()),
            );
        }
        Value::Object(map)
    }
}
