use roxmltree::Attributes;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub(crate) struct Content {
    pub type_name: String,
    pub props: serde_json::Map<String, Value>,
    pub content: Option<Vec<Content>>,
    pub styles: Vec<Style>,
}

impl Content {
    pub fn new() -> Self {
        Content {
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
            match style {
                Style::Bold => styles.insert("bold".to_string(), json!(true)),
                Style::Italic => styles.insert("italic".to_string(), json!(true)),
                Style::Underline => styles.insert("underline".to_string(), json!(true)),
                Style::Strike => styles.insert("strike".to_string(), json!(true)),
            };
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Style {
    Bold,
    Italic,
    Underline,
    Strike,
}

impl Style {
    pub fn name(&self) -> &str {
        match self {
            Style::Bold => "bold",
            Style::Italic => "italic",
            Style::Underline => "underline",
            Style::Strike => "strike",
        }
    }
}

impl From<&str> for Style {
    fn from(s: &str) -> Self {
        match s {
            "bold" => Style::Bold,
            "italic" => Style::Italic,
            "underline" => Style::Underline,
            "strike" => Style::Strike,
            _ => panic!("Invalid style: {}", s),
        }
    }
}
