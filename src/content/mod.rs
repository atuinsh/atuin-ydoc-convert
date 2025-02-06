use basic::BasicContent;
use serde_json::{json, Value};
use table::TableContent;

pub(crate) mod basic;
pub(crate) mod style;
pub(crate) mod table;

#[derive(Clone, Debug)]
pub(crate) enum Content {
    Basic(Vec<BasicContent>),
    Table(TableContent),
}

impl Content {
    pub fn to_json(&self) -> Value {
        match self {
            Content::Basic(content) => {
                json!(content.iter().map(|c| c.to_json()).collect::<Vec<Value>>())
            }
            Content::Table(content) => content.to_json(),
        }
    }
}
