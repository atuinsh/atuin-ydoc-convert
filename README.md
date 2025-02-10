# atuin-ydoc-convert

> **Note:** this is an internal repository, and issues and pull requests will not be accepted.

Converts XML from BlockNote's YJS format to the standard BlockNote JSON format. Uses [roxmltree](https://crates.io/crates/roxmltree) to parse the XML.

## Usage

```rust
let xml_string = // ...
let json = atuin_ydoc_convert::convert_to_json(xml_string).unwrap();
println!("{}", json);
```

## API

### Functions

* `get_fragment_xml(doc: yrs::Doc, fragment_name: String) -> String` \
  Returns well-formatted XML for the fragment with the given name in the given document. Note that this is different than what you would get if you called `get_string()` on the `XmlFragmentRef`; the XML returned by this function is not identical to the XML that BlockNote uses to store documents. It is crafted to be suitable to be parsed by this library.
* `convert_to_json(xml: String) -> Result<String, Error>` \
  Converts the XML to a JSON string.
* `convert_to_json_pretty(xml: String) -> Result<String, Error>` \
  Converts the XML to a pretty-printed JSON string.
* `convert_to_value(xml: String) -> Result<serde_json::Value, Error>` \
  Converts the XML to a `serde_json::Value`.

### Types

* `Error` - Error enum
    * `ParseError(roxmltree::Error)` \
      Wraps errors from `roxmltree` when parsing the XML document.
    * `MalformedDocument(String, roxmltree::TextPos)` \
      Emitted when the XML document has unexpected structure (e.g. not a valid BlockNote document).

## Notes and Exceptions

1. There are some properties that exist on the XML tags that aren't found in BlockNote objects. Since the BlockNote schema is needed to determine which properties are valid for custom block types, this library applies _all_ attributes found on  XML tags to custom BlockNote block objects. Some examples:
    * `<blockcontainer>` nodes always have a `backgroundColor` attribute, but many blocks do not use it as a prop.
    * An empty `content` array is added to all blocks, including ones that don't support content.
2. Since XML encodes all attributes as strings, this library parses them as strings as well. Properties for built-in block types are converted to numerics or booleans as appropriate; for custom blocks, consumers should handle the conversion.
