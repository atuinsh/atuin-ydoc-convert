use std::collections::HashMap;
use std::sync::Arc;
use yrs::types::text::YChange;
use yrs::{Text, Transact, Transaction, Xml, XmlFragment, XmlFragmentRef, XmlOut};

pub(crate) fn get_fragment_xml(doc: yrs::Doc, fragment_name: String) -> String {
    let xml = doc.get_or_insert_xml_fragment(fragment_name);
    let mut out = String::new();
    serialize_xml_fragment(xml, &mut out, &doc.transact());

    out
}

fn serialize_xml_fragment(frag: XmlFragmentRef, out: &mut String, txn: &Transaction) {
    for node in frag.children(txn) {
        serialize_xml_out(node, out, txn);
    }
}

fn serialize_xml_out(elem: XmlOut, out: &mut String, txn: &Transaction) {
    match elem {
        XmlOut::Element(elem) => {
            let tag = elem.tag().to_string();
            out.push_str(&format!("<{}", tag));

            for (name, value) in elem.attributes(txn) {
                out.push_str(&format!(" {}=\"{}\"", name, escape_xml_text(&value)));
            }

            out.push('>');

            for child in elem.children(txn) {
                serialize_xml_out(child, out, txn);
            }

            out.push_str(&format!("</{}>", tag));
        }
        XmlOut::Fragment(frag) => {
            serialize_xml_fragment(frag, out, txn);
        }
        XmlOut::Text(text) => {
            let diffs = text.diff(txn, YChange::identity);
            for diff in diffs {
                if let yrs::Out::Any(yrs::Any::String(s)) = diff.insert {
                    let mut attributes = Vec::new();
                    if let Some(attr_map) = &diff.attributes {
                        attributes.extend(attr_map.iter());
                    }

                    serialize_diff_insert_string(s, &attributes, out);
                }
            }
        }
    }
}

// top-level attrs represent wrapping tags (e.g. "bold", "strike", and "link")
fn serialize_diff_insert_string(
    s: Arc<str>,
    diff_attrs: &[(&Arc<str>, &yrs::Any)],
    out: &mut String,
) {
    for (tag_name, attributes) in diff_attrs.iter() {
        out.push_str(&format!("<{}", tag_name));

        if let yrs::Any::Map(m) = attributes {
            serialize_diff_attr_map(m.clone(), out);
        }

        out.push('>');
    }

    out.push_str(&escape_xml_text(&s));

    for (tag_name, _) in diff_attrs.iter().rev() {
        out.push_str(&format!("</{}>", tag_name));
    }
}

fn serialize_diff_attr_map(m: Arc<HashMap<String, yrs::Any>>, out: &mut String) {
    for (name, value) in m.iter() {
        match value {
            yrs::Any::Null => {
                out.push_str(&format!(" {}=\"\"", name));
            }
            yrs::Any::String(s) => {
                out.push_str(&format!(" {}=\"{}\"", name, escape_xml_text(s)));
            }
            _ => {}
        }
    }
}

fn escape_xml_text(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            '\n' => "&#10;".to_string(),
            '\r' => "&#13;".to_string(),
            '\t' => "&#9;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
