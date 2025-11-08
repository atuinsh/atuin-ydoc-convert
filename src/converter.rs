use roxmltree::{Document, Node, NodeType};
use serde_json::Value;

use crate::{
    block::Block,
    content::{
        basic::BasicContent,
        style::Style,
        table::{TableCell, TableContent, TableRow},
        Content,
    },
};

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(roxmltree::Error),
    MalformedDocument(String, roxmltree::TextPos),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::MalformedDocument(message, pos) => {
                write!(
                    f,
                    "Malformed document: {} at line {} column {}",
                    message, pos.row, pos.col
                )
            }
        }
    }
}

pub(crate) fn convert(xml: String) -> Result<serde_json::Value, Error> {
    let doc = Document::parse(&xml).map_err(Error::ParseError)?;
    let blockgroup = doc.root_element();

    let blocks = convert_blockgroup(blockgroup)?;

    Ok(Value::Array(
        blocks.iter().map(|block| block.to_json()).collect(),
    ))
}

fn convert_blockgroup(blockgroup: Node) -> Result<Vec<Block>, Error> {
    blockgroup
        .children()
        .filter(|child| child.is_element())
        .map(|block_container| convert_block_container(block_container))
        .collect()
}

fn convert_block_container(block_container: Node) -> Result<Block, Error> {
    let mut block = Block::new();
    block.apply_attributes(block_container.attributes());
    let mut children = block_container
        .children()
        .filter(|child| child.is_element());
    let Some(block_elem) = children.next() else {
        return Err(Error::MalformedDocument(
            "blockcontainer with no children".to_string(),
            block_container
                .document()
                .text_pos_at(block_container.range().start),
        ));
    };

    block.type_name = block_elem.tag_name().name().to_string();
    block.apply_attributes(block_elem.attributes());

    if block.type_name.as_str() == "table" {
        return convert_table(block_elem, block);
    }

    // Check if the block has content
    let content = block_elem
        .children()
        .map(|child| convert_content(child, &mut vec![]))
        .collect::<Result<Vec<_>, _>>()?;
    block.content = Some(Content::Basic(content));

    // Check if the block has children
    if let Some(blockgroup) = children.next() {
        block.children = convert_blockgroup(blockgroup)?;
    }

    Ok(block)
}

fn convert_content(node: Node, styles: &mut Vec<Style>) -> Result<BasicContent, Error> {
    match node.node_type() {
        NodeType::Text => {
            let mut content = BasicContent::new();
            content.type_name = "text".to_string();
            content.props.insert("text".to_string(), node.text().into());
            content.styles = styles.clone();
            Ok(content)
        }
        NodeType::Element => match node.tag_name().name() {
            "bold" | "italic" | "underline" | "strike" | "code" | "textColor"
            | "backgroundColor" => {
                // Style tags can have either one text child or one element child.
                // In the case of an element child, the tag could be surrounded by whitespace.
                // This seems to only happen when the XML is formatted with newlines,
                // so we strip whitespace from text nodes.
                let children = node
                    .children()
                    .filter(|child| {
                        child.is_element()
                            || (child.is_text()
                                && !child
                                    .text()
                                    .expect("child.is_text() is true, but child.text() is None")
                                    .trim()
                                    .is_empty())
                    })
                    .collect::<Vec<_>>();
                if children.is_empty() {
                    Err(Error::MalformedDocument(
                        "style tag with no children".to_string(),
                        node.document().text_pos_at(node.range().start),
                    ))
                } else if children.len() > 1 {
                    Err(Error::MalformedDocument(
                        "style tag with more than one child".to_string(),
                        node.document().text_pos_at(node.range().start),
                    ))
                } else {
                    match node.tag_name().name() {
                        "textColor" => {
                            styles.push(Style::TextColor(
                                node.attributes()
                                    .find(|attr| attr.name() == "stringValue")
                                    .map(|attr| attr.value().to_string())
                                    .unwrap_or("default".to_string()),
                            ));
                        }
                        "backgroundColor" => {
                            styles.push(Style::BackgroundColor(
                                node.attributes()
                                    .find(|attr| attr.name() == "stringValue")
                                    .map(|attr| attr.value().to_string())
                                    .unwrap_or("default".to_string()),
                            ));
                        }
                        style => {
                            styles.push(style.try_into().map_err(|e| {
                                Error::MalformedDocument(
                                    e,
                                    node.document().text_pos_at(node.range().start),
                                )
                            })?);
                        }
                    }
                    convert_content(
                        *children
                            .first()
                            .expect("children.is_empty() is false, but children.first() is None"),
                        styles,
                    )
                }
            }
            name => {
                let mut content = BasicContent::new();
                content.type_name = name.to_string();
                content.apply_attributes(node.attributes());
                let children = node.children().collect::<Vec<_>>();
                if !children.is_empty() {
                    content.content = Some(
                        children
                            .iter()
                            .map(|child| convert_content(*child, styles))
                            .collect::<Result<Vec<_>, _>>()?,
                    );
                }
                Ok(content)
            }
        },
        _ => Err(Error::MalformedDocument(
            "Unsupported node type in content position".to_string(),
            node.document().text_pos_at(node.range().start),
        )),
    }
}

fn convert_table(block_elem: Node, mut block: Block) -> Result<Block, Error> {
    let row_elems = block_elem.children().filter(|child| child.is_element());
    let rows = row_elems
        .map(|row_elem| convert_table_row(row_elem))
        .collect::<Result<Vec<_>, _>>()?;

    block.content = Some(Content::Table(TableContent::new(rows)));
    Ok(block)
}

fn convert_table_row(row_elem: Node) -> Result<TableRow, Error> {
    let cell_elems = row_elem.children().filter(|child| child.is_element());
    let cells = cell_elems
        .map(|cell_elem| convert_table_cell(cell_elem))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cells)
}

fn convert_table_cell(cell_elem: Node) -> Result<TableCell, Error> {
    let mut cell = TableCell::new();

    for attr in cell_elem.attributes() {
        match attr.name() {
            "colspan" => {
                cell.colspan = attr.value().parse::<u32>().map_err(|_| {
                    Error::MalformedDocument(
                        "Invalid colspan".to_string(),
                        cell_elem.document().text_pos_at(cell_elem.range().start),
                    )
                })?;
            }
            "rowspan" => {
                cell.rowspan = attr.value().parse::<u32>().map_err(|_| {
                    Error::MalformedDocument(
                        "Invalid rowspan".to_string(),
                        cell_elem.document().text_pos_at(cell_elem.range().start),
                    )
                })?;
            }
            "colwidth" => {
                cell.colwidth = Some(
                    attr.value()
                        .trim_start_matches("[")
                        .trim_end_matches("]")
                        .parse::<u32>()
                        .map_err(|_| {
                            Error::MalformedDocument(
                                "Invalid colwidth".to_string(),
                                cell_elem.document().text_pos_at(cell_elem.range().start),
                            )
                        })?,
                );
            }
            _ => {}
        }
    }

    let paragraph_elem = cell_elem
        .first_element_child()
        .ok_or(Error::MalformedDocument(
            "table cell with no table paragraph".to_string(),
            cell_elem.document().text_pos_at(cell_elem.range().start),
        ))?;

    let content = paragraph_elem
        .children()
        .map(|child| convert_content(child, &mut vec![]))
        .collect::<Result<Vec<_>, _>>()?;
    cell.content = content;

    Ok(cell)
}
