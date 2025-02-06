use serde_json::{json, Value};

use super::basic::BasicContent;

pub(crate) type TableRow = Vec<TableCell>;
pub(crate) type TableCell = Vec<BasicContent>;

#[derive(Clone, Debug)]
pub(crate) struct TableContent {
    pub column_widths: Vec<Option<u32>>,
    pub rows: Vec<TableRow>,
}

impl TableContent {
    pub fn new(rows: Vec<TableRow>) -> Self {
        TableContent {
            column_widths: Vec::new(),
            rows,
        }
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), json!("tableContent"));
        map.insert("columnWidths".to_string(), json!(self.column_widths));
        map.insert(
            "rows".to_string(),
            json!(self
                .rows
                .iter()
                .map(|row| {
                    let cells = row
                        .iter()
                        .map(|cell| {
                            json!(cell
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
