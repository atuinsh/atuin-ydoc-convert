use roxmltree::Attributes;
use serde_json::{json, Value};

use crate::content::Content;

#[derive(Clone, Debug)]
pub(crate) struct Block {
    pub id: String,
    pub type_name: String,
    pub props: serde_json::Map<String, Value>,
    pub content: Option<Vec<Content>>,
    pub children: Vec<Block>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            id: String::new(),
            type_name: String::new(),
            props: serde_json::Map::new(),
            content: None,
            children: Vec::new(),
        }
    }

    pub fn apply_attributes(&mut self, attributes: Attributes) {
        for attr in attributes {
            match attr.name() {
                "id" => {
                    self.id = attr.value().to_string();
                }
                name => {
                    self.props.insert(name.to_string(), json!(attr.value()));
                }
            }
        }
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("id".to_string(), json!(self.id));
        let type_name = match self.type_name.as_str() {
            "bulletlistitem" => "bulletListItem",
            "checklistitem" => "checkListItem",
            "numberedlistitem" => "numberedListItem",
            _ => &self.type_name,
        };
        map.insert("type".to_string(), json!(type_name));
        map.insert("props".to_string(), Value::Object(self.props.clone()));
        if let Some(content) = &self.content {
            map.insert(
                "content".to_string(),
                json!(content.iter().map(|c| c.to_json()).collect::<Vec<Value>>()),
            );
        } else {
            map.insert("content".to_string(), json!([]));
        }
        if !self.children.is_empty() {
            let children: Vec<Value> = self.children.iter().map(|child| child.to_json()).collect();
            map.insert("children".to_string(), json!(children));
        } else {
            map.insert("children".to_string(), json!([]));
        }

        Value::Object(map)
    }
}
