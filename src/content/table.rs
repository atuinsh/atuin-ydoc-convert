use serde_json::{json, Value};

use super::basic::BasicContent;

pub(crate) type TableRow = Vec<TableCell>;

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
    pub content: Vec<BasicContent>,
    pub colspan: u32,
    pub rowspan: u32,
    pub colwidth: Option<u32>,
}

impl TableCell {
    pub fn new() -> Self {
        TableCell {
            content: Vec::new(),
            colspan: 1,
            rowspan: 1,
            colwidth: None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TableContent(Vec<TableRow>);

impl TableContent {
    pub fn new(rows: Vec<TableRow>) -> Self {
        TableContent(rows)
    }

    pub fn to_json(&self) -> Value {
        let column_widths = self
            .0
            .iter()
            .next()
            .map(|row| row.iter().map(|cell| cell.colwidth).collect::<Vec<_>>())
            .unwrap_or(vec![]);

        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), json!("tableContent"));
        map.insert("columnWidths".to_string(), json!(column_widths));
        map.insert(
            "rows".to_string(),
            json!(self
                .0
                .iter()
                .map(|row| {
                    let cells = row
                        .iter()
                        .map(|cell| {
                            json!(cell
                                .content
                                .iter()
                                .map(|content| content.to_json())
                                .collect::<Vec<Value>>())
                        })
                        .collect::<Vec<_>>();
                    let mut map = serde_json::Map::new();
                    map.insert("cells".to_string(), json!(cells));
                    Value::Object(map)
                })
                .collect::<Vec<_>>()),
        );
        Value::Object(map)
    }
}
