use roxmltree::{Document, Node, NodeType};
use serde_json::Value;

use crate::{
    block::Block,
    content::{Content, Style},
};

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(roxmltree::Error),
    MalformedDocument(String, roxmltree::TextPos),
}

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
    let doc = Document::parse(&xml).map_err(|e| Error::ParseError(e))?;
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

    // Check if the block has content
    let content = block_elem
        .children()
        .map(|child| convert_content(child, &mut vec![]))
        .collect::<Result<Vec<_>, _>>()?;
    block.content = Some(content);

    // Check if the block has children
    if let Some(blockgroup) = children.next() {
        block.children = convert_blockgroup(blockgroup)?;
    }

    Ok(block)
}

fn convert_content(node: Node, styles: &mut Vec<Style>) -> Result<Content, Error> {
    match node.node_type() {
        NodeType::Text => {
            let mut content = Content::new();
            content.type_name = "text".to_string();
            content.props.insert("text".to_string(), node.text().into());
            content.styles = styles.clone();
            Ok(content)
        }
        NodeType::Element => match node.tag_name().name() {
            "bold" | "italic" | "underline" | "strike" => {
                // Style tags can have either one text child or one element child.
                // In the case of an element child, the tag could be surrounded by whitespace.
                // This seems to only happen when the XML is formatted with newlines,
                // so we strip whitespace from text nodes.
                let children = node
                    .children()
                    .filter(|child| {
                        child.is_element()
                            || (child.is_text() && !child.text().unwrap().trim().is_empty())
                    })
                    .collect::<Vec<_>>();
                if children.len() == 0 {
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
                    styles.push(node.tag_name().name().into());
                    convert_content(*children.first().unwrap(), styles)
                }
            }
            name => {
                let mut content = Content::new();
                content.type_name = name.to_string();
                content.apply_attributes(node.attributes());
                let children = node.children().collect::<Vec<_>>();
                if children.len() > 0 {
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
        other => {
            return Err(Error::MalformedDocument(
                "Unsupported node type in content position".to_string(),
                node.document().text_pos_at(node.range().start),
            ));
        }
    }
}
